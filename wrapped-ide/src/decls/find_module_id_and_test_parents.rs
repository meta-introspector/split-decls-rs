macro_rules! deps {
    () => {
        TestItemKind!();
        TestItem!();
        NavigationTarget!();
    };
}

macro_rules! find_module_id_and_test_parents {
    () => {
        deps!();
        fn find_module_id_and_test_parents (sema : & Semantics < '_ , RootDatabase > , module : Module ,) -> Option < (Vec < TestItem > , String) > { let Some (parent) = module . parent (sema . db) else { let name = module . krate () . display_name (sema . db) ? . to_string () ; return Some ((vec ! [TestItem { id : name . clone () , kind : TestItemKind :: Crate (module . krate () . into ()) , label : name . clone () , parent : None , file : None , text_range : None , runnable : None , }] , name ,)) ; } ; let (mut r , mut id) = find_module_id_and_test_parents (sema , parent) ? ; let parent = Some (id . clone ()) ; id += "::" ; let module_name = & module . name (sema . db) ; let module_name = module_name . as_ref () . map (| n | n . as_str ()) . unwrap_or ("[mod without name]") ; id += module_name ; let nav = NavigationTarget :: from_module_to_decl (sema . db , module) . call_site ; r . push (TestItem { id : id . clone () , kind : TestItemKind :: Module , label : module_name . to_owned () , parent , file : Some (nav . file_id) , text_range : Some (nav . focus_or_full_range ()) , runnable : None , }) ; Some ((r , id)) }
    };
}

find_module_id_and_test_parents!()