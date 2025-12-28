macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! orig_range_with_focus_r {
    () => {
        deps!();
        pub (crate) fn orig_range_with_focus_r (db : & RootDatabase , hir_file : HirFileId , value : TextRange , focus_range : Option < TextRange > ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { let Some (name) = focus_range else { return orig_range_r (db , hir_file , value) } ; let call_kind = | | db . lookup_intern_macro_call (hir_file . macro_file () . unwrap ()) . kind ; let def_range = | | db . lookup_intern_macro_call (hir_file . macro_file () . unwrap ()) . def . definition_range (db) ; let value_range = InFile :: new (hir_file , value) . original_node_file_range_opt (db) ; let ((call_site_range , call_site_focus) , def_site) = match InFile :: new (hir_file , name) . original_node_file_range_opt (db) { Some ((focus_range , ctxt)) if ctxt . is_root () => { ((match value_range { Some ((range , ctxt)) if ctxt . is_root () && range . file_id == focus_range . file_id && range . range . contains_range (focus_range . range) => { range } _ => { let kind = call_kind () ; let range = kind . clone () . original_call_range_with_input (db) ; if range . file_id == focus_range . file_id && range . range . contains_range (focus_range . range) { range } else { kind . original_call_range (db) } } } , Some (focus_range) ,) , None ,) } Some ((focus_range , _ctxt)) => { match value_range { Some ((range , ctxt)) if ctxt . is_root () => ((range , None) , { let (def_site , _) = def_range () . original_node_file_range (db) ; (def_site . file_id == focus_range . file_id && def_site . range . contains_range (focus_range . range)) . then_some ((def_site , Some (focus_range))) } ,) , _ => ((call_kind () . original_call_range (db) , None) , Some ((focus_range , Some (focus_range))) ,) , } } None => return orig_range_r (db , hir_file , value) , } ; UpmappingResult { call_site : (call_site_range . into_file_id (db) , call_site_focus . and_then (| hir :: FileRange { file_id , range } | { if call_site_range . file_id == file_id && call_site_range . range . contains_range (range) { Some (range) } else { None } }) ,) , def_site : def_site . map (| (def_site_range , def_site_focus) | { (def_site_range . into_file_id (db) , def_site_focus . and_then (| hir :: FileRange { file_id , range } | { if def_site_range . file_id == file_id && def_site_range . range . contains_range (range) { Some (range) } else { None } }) ,) }) , } }
    };
}

orig_range_with_focus_r!()