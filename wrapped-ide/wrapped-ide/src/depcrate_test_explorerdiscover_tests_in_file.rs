// Generated macro for discover_tests_in_file (function)
macro_rules! Depcrate_test_explorerdiscover_tests_in_file {
() => {
// Module: crate::test_explorer
// Provides: {"discover_tests_in_file"}
// Dependencies: {}
pub (crate) fn discover_tests_in_file (db : & RootDatabase , file_id : FileId) -> Vec < TestItem > { let sema = Semantics :: new (db) ; let Some (module) = sema . file_to_module_def (file_id) else { return vec ! [] } ; let Some ((mut tests , id)) = find_module_id_and_test_parents (& sema , module) else { return vec ! [] ; } ; tests . extend (discover_tests_in_module (db , module , id , true)) ; tests }
};
}
