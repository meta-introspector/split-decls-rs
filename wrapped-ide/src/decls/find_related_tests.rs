macro_rules! deps {
    () => {
        Runnable!();
    };
}

macro_rules! find_related_tests {
    () => {
        deps!();
        fn find_related_tests (sema : & Semantics < '_ , RootDatabase > , syntax : & SyntaxNode , position : FilePosition , search_scope : Option < SearchScope > , tests : & mut FxIndexSet < Runnable > ,) { let defs = match references :: find_defs (sema , syntax , position . offset) { Some (defs) => defs , None => return , } ; for def in defs { let defs = def . usages (sema) . set_scope (search_scope . as_ref ()) . all () . references . into_values () . flatten () ; for ref_ in defs { let name_ref = match ref_ . name { FileReferenceNode :: NameRef (name_ref) => name_ref , _ => continue , } ; if let Some (fn_def) = sema . ancestors_with_macros (name_ref . syntax () . clone ()) . find_map (ast :: Fn :: cast) { if let Some (runnable) = as_test_runnable (sema , & fn_def) { tests . insert (runnable) ; } else if let Some (module) = parent_test_module (sema , & fn_def) { find_related_tests_in_module (sema , syntax , & fn_def , & module , tests) ; } } } } }
    };
}

find_related_tests!()