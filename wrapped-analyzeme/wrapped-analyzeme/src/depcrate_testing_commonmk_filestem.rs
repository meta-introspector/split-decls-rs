// Generated macro for mk_filestem (function)
macro_rules! Depcrate_testing_commonmk_filestem {
() => {
// Module: crate::testing_common
// Provides: {"mk_filestem"}
// Dependencies: {}
fn mk_filestem (file_name_stem : & str) -> PathBuf { let mut path = PathBuf :: new () ; path . push ("test-tmp") ; path . push ("end_to_end_serialization") ; path . push (file_name_stem) ; path }
};
}
