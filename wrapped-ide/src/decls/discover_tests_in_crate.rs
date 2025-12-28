macro_rules! deps {
    () => {
        TestItem!();
        TestItemKind!();
    };
}

macro_rules! discover_tests_in_crate {
    () => {
        deps!();
        pub (crate) fn discover_tests_in_crate (db : & RootDatabase , crate_id : base_db :: Crate ,) -> Vec < TestItem > { if ! crate_id . data (db) . origin . is_local () { return vec ! [] ; } let Some (crate_test_id) = & crate_id . extra_data (db) . display_name else { return vec ! [] ; } ; let kind = TestItemKind :: Crate (crate_id) ; let crate_test_id = crate_test_id . to_string () ; let crate_id : Crate = crate_id . into () ; let module = crate_id . root_module () ; let mut r = vec ! [TestItem { id : crate_test_id . clone () , kind , label : crate_test_id . clone () , parent : None , file : None , text_range : None , runnable : None , }] ; r . extend (discover_tests_in_module (db , module , crate_test_id , false)) ; r }
    };
}

discover_tests_in_crate!();