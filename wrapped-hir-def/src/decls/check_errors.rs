macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! check_errors {
    () => {
        deps!();
        # [track_caller] fn check_errors (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let db = TestDB :: with_files (ra_fixture) ; let krate = db . fetch_test_crate () ; let def_map = crate_def_map (& db , krate) ; let errors = def_map . modules () . flat_map (| module | module . 1 . scope . all_macro_calls ()) . filter_map (| macro_call | { let errors = db . parse_macro_expansion_error (macro_call) ? ; let errors = errors . err . as_ref () ? . render_to_string (& db) ; let macro_loc = db . lookup_intern_macro_call (macro_call) ; let ast_id = match macro_loc . kind { MacroCallKind :: FnLike { ast_id , .. } => ast_id . map (| it | it . erase ()) , MacroCallKind :: Derive { ast_id , .. } => ast_id . map (| it | it . erase ()) , MacroCallKind :: Attr { ast_id , .. } => ast_id . map (| it | it . erase ()) , } ; let editioned_file_id = ast_id . file_id . file_id () . expect ("macros inside macros are not supported") ; let ast = db . parse (editioned_file_id) . syntax_node () ; let ast_id_map = db . ast_id_map (ast_id . file_id) ; let node = ast_id_map . get_erased (ast_id . value) . to_node (& ast) ; Some ((node . text_range () , errors)) }) . sorted_unstable_by_key (| (range , _) | range . start ()) . format_with ("\n" , | (range , err) , format | format (& format_args ! ("{range:?}: {err}"))) . to_string () ; expect . assert_eq (& errors) ; }
    };
}

check_errors!()