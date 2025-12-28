macro_rules! deps {
    () => {
        TestItem!();
        TestItemKind!();
        NavigationTarget!();
    };
}

macro_rules! discover_tests_in_module {
    () => {
        deps!();
        fn discover_tests_in_module (db : & RootDatabase , module : Module , prefix_id : String , only_in_this_file : bool ,) -> Vec < TestItem > { let sema = Semantics :: new (db) ; let mut r = vec ! [] ; for c in module . children (db) { let module_name = c . name (db) . as_ref () . map (| n | n . as_str () . to_owned ()) . unwrap_or_else (| | "[mod without name]" . to_owned ()) ; let module_id = format ! ("{prefix_id}::{module_name}") ; let module_children = discover_tests_in_module (db , c , module_id . clone () , only_in_this_file) ; if ! module_children . is_empty () { let nav = NavigationTarget :: from_module_to_decl (sema . db , c) . call_site ; r . push (TestItem { id : module_id , kind : TestItemKind :: Module , label : module_name , parent : Some (prefix_id . clone ()) , file : Some (nav . file_id) , text_range : Some (nav . focus_or_full_range ()) , runnable : None , }) ; if ! only_in_this_file || c . is_inline (db) { r . extend (module_children) ; } } } for def in module . declarations (db) { let ModuleDef :: Function (f) = def else { continue ; } ; if ! f . is_test (db) { continue ; } let nav = f . try_to_nav (& sema) . map (| r | r . call_site) ; let fn_name = f . name (db) . as_str () . to_owned () ; r . push (TestItem { id : format ! ("{prefix_id}::{fn_name}") , kind : TestItemKind :: Function , label : fn_name , parent : Some (prefix_id . clone ()) , file : nav . as_ref () . map (| n | n . file_id) , text_range : nav . as_ref () . map (| n | n . focus_or_full_range ()) , runnable : runnable_fn (& sema , f) , }) ; } r }
    };
}

discover_tests_in_module!()