// Generated macro for sysroot (function)
macro_rules! Depcrate_pathssysroot {
() => {
// Module: crate::paths
// Provides: {"sysroot"}
// Dependencies: {}
# [doc = " Path to `rustc`s sysroot"] pub fn sysroot () -> String { let output = Command :: new ("rustc") . arg ("--print=sysroot") . output () . expect ("rustc to run") ; assert ! (output . status . success ()) ; let sysroot = String :: from_utf8 (output . stdout) . unwrap () ; sysroot . trim () . to_string () }
};
}
