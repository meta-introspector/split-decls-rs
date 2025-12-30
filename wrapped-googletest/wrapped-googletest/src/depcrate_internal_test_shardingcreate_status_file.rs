// Generated macro for create_status_file (function)
macro_rules! Depcrate_internal_test_shardingcreate_status_file {
() => {
// Module: crate::internal::test_sharding
// Provides: {"create_status_file"}
// Dependencies: {}
fn create_status_file (path : & Path) -> std :: io :: Result < () > { if let Some (parent) = path . parent () { fs :: create_dir_all (parent) ? ; } File :: create (path) . map (| _ | ()) }
};
}
