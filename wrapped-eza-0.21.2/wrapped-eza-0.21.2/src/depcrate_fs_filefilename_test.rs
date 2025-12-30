// Generated macro for filename_test (module)
macro_rules! Depcrate_fs_filefilename_test {
() => {
// Module: crate::fs::file
// Provides: {"filename_test"}
// Dependencies: {}
# [cfg (test)] mod filename_test { use super :: File ; use std :: path :: Path ; # [test] fn file () { assert_eq ! ("fester.dat" , File :: filename (Path :: new ("fester.dat"))) ; } # [test] fn no_path () { assert_eq ! ("foo.wha" , File :: filename (Path :: new ("/var/cache/foo.wha"))) ; } # [test] fn here () { assert_eq ! ("." , File :: filename (Path :: new ("."))) ; } # [test] fn there () { assert_eq ! (".." , File :: filename (Path :: new (".."))) ; } # [test] fn everywhere () { assert_eq ! (".." , File :: filename (Path :: new ("./.."))) ; } # [test] # [cfg (unix)] fn topmost () { assert_eq ! ("/" , File :: filename (Path :: new ("/"))) ; } }
};
}
