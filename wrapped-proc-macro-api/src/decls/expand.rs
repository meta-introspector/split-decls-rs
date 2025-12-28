macro_rules! deps {
    () => {
        ServerError!();
        ProcMacro!();
    };
}

macro_rules! expand {
    () => {
        deps!();
        pub (crate) fn expand (proc_macro : & ProcMacro , subtree : tt :: SubtreeView < '_ , Span > , attr : Option < tt :: SubtreeView < '_ , Span > > , env : Vec < (String , String) > , def_site : Span , call_site : Span , mixed_site : Span , current_dir : String ,) -> Result < Result < tt :: TopSubtree < span :: SpanData < span :: SyntaxContext > > , String > , crate :: ServerError > { let version = proc_macro . process . version () ; let mut span_data_table = SpanDataIndexMap :: default () ; let def_site = span_data_table . insert_full (def_site) . 0 ; let call_site = span_data_table . insert_full (call_site) . 0 ; let mixed_site = span_data_table . insert_full (mixed_site) . 0 ; let task = ExpandMacro { data : ExpandMacroData { macro_body : FlatTree :: new (subtree , version , & mut span_data_table) , macro_name : proc_macro . name . to_string () , attributes : attr . map (| subtree | FlatTree :: new (subtree , version , & mut span_data_table)) , has_global_spans : ExpnGlobals { serialize : version >= version :: HAS_GLOBAL_SPANS , def_site , call_site , mixed_site , } , span_data_table : if proc_macro . process . rust_analyzer_spans () { serialize_span_data_index_map (& span_data_table) } else { Vec :: new () } , } , lib : proc_macro . dylib_path . to_path_buf () . into () , env , current_dir : Some (current_dir) , } ; let response = send_task (& proc_macro . process , Request :: ExpandMacro (Box :: new (task))) ? ; match response { Response :: ExpandMacro (it) => Ok (it . map (| tree | { let mut expanded = FlatTree :: to_subtree_resolved (tree , version , & span_data_table) ; if proc_macro . needs_fixup_change () { proc_macro . change_fixup_to_match_old_server (& mut expanded) ; } expanded }) . map_err (| msg | msg . 0)) , Response :: ExpandMacroExtended (it) => Ok (it . map (| resp | { let mut expanded = FlatTree :: to_subtree_resolved (resp . tree , version , & deserialize_span_data_index_map (& resp . span_data_table) ,) ; if proc_macro . needs_fixup_change () { proc_macro . change_fixup_to_match_old_server (& mut expanded) ; } expanded }) . map_err (| msg | msg . 0)) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
    };
}

expand!();