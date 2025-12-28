macro_rules! deps {
    () => {
        Error!();
        Tree!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Tree { # [doc = " Validate the correctness of this instance. If `use_objects` is true, then `objects` will be used to access all objects."] pub fn verify (& self , use_objects : bool , objects : impl gix_object :: Find) -> Result < () , Error > { fn verify_recursive (parent_id : gix_hash :: ObjectId , children : & [Tree] , mut object_buf : Option < & mut Vec < u8 > > , objects : & impl gix_object :: Find ,) -> Result < Option < u32 > , Error > { if children . is_empty () { return Ok (None) ; } let mut entries = 0 ; let mut prev = None :: < & Tree > ; for child in children { entries += child . num_entries . unwrap_or (0) ; if let Some (prev) = prev { if prev . name . cmp (& child . name) != Ordering :: Less { return Err (Error :: OutOfOrder { parent_id , previous_path : prev . name . as_bstr () . into () , current_path : child . name . as_bstr () . into () , }) ; } } prev = Some (child) ; } if let Some (buf) = object_buf . as_mut () { let tree_entries = objects . find_tree_iter (& parent_id , buf) ? ; let mut num_entries = 0 ; for entry in tree_entries . filter_map (Result :: ok) . filter (| e | e . mode . is_tree ()) { children . binary_search_by (| e | e . name . as_bstr () . cmp (entry . filename)) . map_err (| _ | Error :: MissingTreeDirectory { parent_id , entry_id : entry . oid . to_owned () , name : entry . filename . to_owned () , }) ? ; num_entries += 1 ; } if num_entries != children . len () { return Err (Error :: TreeNodeChildcountMismatch { oid : parent_id , expected_childcount : num_entries , actual_childcount : children . len () , }) ; } } for child in children { # [allow (clippy :: needless_option_as_deref)] let actual_num_entries = verify_recursive (child . id , & child . children , object_buf . as_deref_mut () , objects) ? ; if let Some ((actual , num_entries)) = actual_num_entries . zip (child . num_entries) { if actual > num_entries { return Err (Error :: EntriesCount { actual , expected : num_entries , }) ; } } } Ok (entries . into ()) } let _span = gix_features :: trace :: coarse ! ("gix_index::extension::Tree::verify()") ; if ! self . name . is_empty () { return Err (Error :: RootWithName { name : self . name . as_bstr () . into () , }) ; } let mut buf = Vec :: new () ; let declared_entries = verify_recursive (self . id , & self . children , use_objects . then_some (& mut buf) , & objects) ? ; if let Some ((actual , num_entries)) = declared_entries . zip (self . num_entries) { if actual > num_entries { return Err (Error :: EntriesCount { actual , expected : num_entries , }) ; } } Ok (()) } }
    };
}

impl_35!()