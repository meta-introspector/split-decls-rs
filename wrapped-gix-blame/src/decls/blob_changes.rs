macro_rules! deps {
    () => {
        Statistics!();
        Error!();
        Change!();
    };
}

macro_rules! blob_changes {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn blob_changes (odb : impl gix_object :: Find + gix_object :: FindHeader , resource_cache : & mut gix_diff :: blob :: Platform , oid : ObjectId , previous_oid : ObjectId , file_path : & BStr , previous_file_path : & BStr , diff_algorithm : imara_diff :: Algorithm , stats : & mut Statistics ,) -> Result < Vec < Change > , Error > { resource_cache . set_resource (previous_oid , gix_object :: tree :: EntryKind :: Blob , previous_file_path , gix_diff :: blob :: ResourceKind :: OldOrSource , & odb ,) ? ; resource_cache . set_resource (oid , gix_object :: tree :: EntryKind :: Blob , file_path , gix_diff :: blob :: ResourceKind :: NewOrDestination , & odb ,) ? ; let outcome = resource_cache . prepare_diff () ? ; let input = imara_diff :: InternedInput :: new (tokens_for_diffing (outcome . old . data . as_slice () . unwrap_or_default ()) , tokens_for_diffing (outcome . new . data . as_slice () . unwrap_or_default ()) ,) ; let number_of_lines_in_destination = input . after . len () as u32 ; let mut diff = imara_diff :: Diff :: compute (diff_algorithm , & input) ; diff . postprocess_lines (& input) ; let mut hunks : Vec < Change > = Vec :: new () ; let mut last_seen_after_end : u32 = 0 ; for hunk in diff . hunks () { if hunk . after . start > last_seen_after_end { hunks . push (Change :: Unchanged (last_seen_after_end .. hunk . after . start)) ; } if ! hunk . after . is_empty () { hunks . push (Change :: AddedOrReplaced (hunk . after . start .. hunk . after . end , (hunk . before . end - hunk . before . start) as u32 ,)) ; } else if ! hunk . before . is_empty () { hunks . push (Change :: Deleted (hunk . after . start as u32 , (hunk . before . end - hunk . before . start) as u32 ,)) ; } last_seen_after_end = hunk . after . end ; } if number_of_lines_in_destination > last_seen_after_end { hunks . push (Change :: Unchanged (last_seen_after_end .. number_of_lines_in_destination)) ; } stats . blobs_diffed += 1 ; Ok (hunks) }
    };
}

blob_changes!()