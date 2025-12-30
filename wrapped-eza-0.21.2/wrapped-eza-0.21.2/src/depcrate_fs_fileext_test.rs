// Generated macro for ext_test (module)
macro_rules! Depcrate_fs_fileext_test {
() => {
// Module: crate::fs::file
// Provides: {"ext_test"}
// Dependencies: {}
# [cfg (test)] mod ext_test { use super :: File ; use std :: path :: Path ; # [test] fn extension () { assert_eq ! (Some ("dat" . to_string ()) , File :: ext (Path :: new ("fester.dat"))) ; } # [test] fn dotfile () { assert_eq ! (Some ("vimrc" . to_string ()) , File :: ext (Path :: new (".vimrc"))) ; } # [test] fn no_extension () { assert_eq ! (None , File :: ext (Path :: new ("jarlsberg"))) ; } }
};
}
