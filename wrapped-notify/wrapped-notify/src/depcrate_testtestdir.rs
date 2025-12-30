// Generated macro for testdir (function)
macro_rules! Depcrate_testtestdir {
() => {
// Module: crate::test
// Provides: {"testdir"}
// Dependencies: {}
# [doc = " Creates a [`TestDir`]"] pub fn testdir () -> TestDir { let dir = tempfile :: tempdir () . expect ("Unable to create tempdir") ; let path = std :: fs :: canonicalize (dir . path ()) . unwrap_or_else (| e | { panic ! ("unable to canonicalize tempdir path {:?}: {e:?}" , dir . path ()) }) ; TestDir { _dir : dir , path } }
};
}
