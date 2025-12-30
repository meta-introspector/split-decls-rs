// Generated macro for find_crate_by_id (function)
macro_rules! Depcrate_test_explorerfind_crate_by_id {
() => {
// Module: crate::test_explorer
// Provides: {"find_crate_by_id"}
// Dependencies: {}
fn find_crate_by_id (db : & RootDatabase , crate_id : & str) -> Option < base_db :: Crate > { db . all_crates () . iter () . copied () . find (| & id | { id . data (db) . origin . is_local () && id . extra_data (db) . display_name . as_ref () . is_some_and (| x | x . to_string () == crate_id) }) }
};
}
