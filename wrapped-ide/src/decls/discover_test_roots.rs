macro_rules! deps {
    () => {
        TestItem!();
        TestItemKind!();
    };
}

macro_rules! discover_test_roots {
    () => {
        deps!();
        pub (crate) fn discover_test_roots (db : & RootDatabase) -> Vec < TestItem > { db . all_crates () . iter () . copied () . filter (| & id | id . data (db) . origin . is_local ()) . filter_map (| id | { let test_id = id . extra_data (db) . display_name . as_ref () ? . to_string () ; Some (TestItem { kind : TestItemKind :: Crate (id) , label : test_id . clone () , id : test_id , parent : None , file : None , text_range : None , runnable : None , }) }) . collect () }
    };
}

discover_test_roots!();