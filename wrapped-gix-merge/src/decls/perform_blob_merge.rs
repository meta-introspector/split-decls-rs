macro_rules! deps {
    () => {
        ConflictMapping!();
        Labels!();
        Options!();
        Error!();
        Pick!();
        ResourceKind!();
        Resolution!();
        Platform!();
    };
}

macro_rules! perform_blob_merge {
    () => {
        deps!();
        # [doc = " Perform a merge between two blobs and return the result of its object id."] # [allow (clippy :: too_many_arguments)] pub fn perform_blob_merge < E > (mut labels : crate :: blob :: builtin_driver :: text :: Labels < '_ > , objects : & impl gix_object :: FindObjectOrHeader , blob_merge : & mut crate :: blob :: Platform , buf : & mut Vec < u8 > , write_blob_to_odb : & mut impl FnMut (& [u8]) -> Result < ObjectId , E > , (our_location , our_id , our_mode) : (& BString , ObjectId , EntryMode) , (their_location , their_id , their_mode) : (& BString , ObjectId , EntryMode) , (previous_location , previous_id , previous_mode) : (& BString , ObjectId , EntryMode) , (extra_markers , outer_side) : (u8 , ConflictMapping) , options : & Options ,) -> Result < (ObjectId , crate :: blob :: Resolution) , Error > where E : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { if our_id == their_id { debug_assert_ne ! (our_mode , their_mode , "BUG: we must think anything has to be merged if the modes and the ids are the same") ; return Ok ((their_id , crate :: blob :: Resolution :: Complete)) ; } if matches ! (our_mode . kind () , EntryKind :: Link) && matches ! (their_mode . kind () , EntryKind :: Link) { let (pick , resolution) = crate :: blob :: builtin_driver :: binary (options . symlink_conflicts) ; let (our_id , their_id) = match outer_side { ConflictMapping :: Original => (our_id , their_id) , ConflictMapping :: Swapped => (their_id , our_id) , } ; let id = match pick { Pick :: Ancestor => previous_id , Pick :: Ours => our_id , Pick :: Theirs => their_id , } ; return Ok ((id , resolution)) ; } let (our_kind , their_kind) = match outer_side { ConflictMapping :: Original => (ResourceKind :: CurrentOrOurs , ResourceKind :: OtherOrTheirs) , ConflictMapping :: Swapped => (ResourceKind :: OtherOrTheirs , ResourceKind :: CurrentOrOurs) , } ; blob_merge . set_resource (our_id , our_mode . kind () , our_location . as_bstr () , our_kind , objects) ? ; blob_merge . set_resource (their_id , their_mode . kind () , their_location . as_bstr () , their_kind , objects ,) ? ; blob_merge . set_resource (previous_id , previous_mode . kind () , previous_location . as_bstr () , ResourceKind :: CommonAncestorOrBase , objects ,) ? ; fn combined (side : & BStr , location : & BString) -> BString { let mut buf = side . to_owned () ; buf . push_byte (b':') ; buf . push_str (location) ; buf } if outer_side . is_swapped () { (labels . current , labels . other) = (labels . other , labels . current) ; } let (ancestor , current , other) ; let labels = if our_location == their_location { labels } else { ancestor = labels . ancestor . map (| side | combined (side , previous_location)) ; current = labels . current . map (| side | combined (side , our_location)) ; other = labels . other . map (| side | combined (side , their_location)) ; crate :: blob :: builtin_driver :: text :: Labels { ancestor : ancestor . as_ref () . map (| n | n . as_bstr ()) , current : current . as_ref () . map (| n | n . as_bstr ()) , other : other . as_ref () . map (| n | n . as_bstr ()) , } } ; let prep = blob_merge . prepare_merge (objects , with_extra_markers (options , extra_markers)) ? ; let (pick , resolution) = prep . merge (buf , labels , & options . blob_merge_command_ctx) ? ; let merged_blob_id = prep . id_by_pick (pick , buf , write_blob_to_odb) . map_err (| err | Error :: WriteBlobToOdb (err . into ())) ? . ok_or (Error :: MergeResourceNotFound) ? ; Ok ((merged_blob_id , resolution)) }
    };
}

perform_blob_merge!()