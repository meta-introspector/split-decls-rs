// Generated macro for discover_test_roots (function)
macro_rules! Depcrate_test_explorerdiscover_test_roots {
() => {
// Module: crate::test_explorer
// Provides: {"discover_test_roots"}
// Dependencies: {}
pub (crate) fn discover_test_roots (db : & RootDatabase) -> Vec < TestItem > { db . all_crates () . iter () . copied () . filter (| & id | id . data (db) . origin . is_local ()) . filter_map (| id | { let test_id = id . extra_data (db) . display_name . as_ref () ? . to_string () ; Some (TestItem { kind : TestItemKind :: Crate (id) , label : test_id . clone () , id : test_id , parent : None , file : None , text_range : None , runnable : None , }) }) . collect () }
};
}
