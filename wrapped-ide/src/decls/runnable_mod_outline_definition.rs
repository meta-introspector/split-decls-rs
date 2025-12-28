macro_rules! deps {
    () => {
        Runnable!();
        UpdateTest!();
        RunnableKind!();
    };
}

macro_rules! runnable_mod_outline_definition {
    () => {
        deps!();
        # [doc = " Creates a test mod runnable for outline modules at the top of their definition."] fn runnable_mod_outline_definition (sema : & Semantics < '_ , RootDatabase > , def : hir :: Module ,) -> Option < Runnable > { def . as_source_file_id (sema . db) ? ; if ! has_test_function_or_multiple_test_submodules (sema , & def , has_cfg_test (def . attrs (sema . db))) { return None ; } let path = def . path_to_root (sema . db) . into_iter () . rev () . filter_map (| module | { module . name (sema . db) . map (| mod_name | { mod_name . display (sema . db , module . krate () . edition (sema . db)) . to_string () }) }) . join ("::") ; let attrs = def . attrs (sema . db) ; let cfg = attrs . cfg () ; let mod_source = sema . module_definition_node (def) ; let mod_syntax = mod_source . file_syntax (sema . db) ; let file_range = hir :: FileRange { file_id : mod_source . file_id . original_file (sema . db) , range : mod_syntax . text_range () , } ; let update_test = UpdateTest :: find_snapshot_macro (sema , file_range) ; Some (Runnable { use_name_in_title : false , nav : def . to_nav (sema . db) . call_site () , kind : RunnableKind :: TestMod { path } , cfg , update_test , }) }
    };
}

runnable_mod_outline_definition!();