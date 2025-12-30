// Generated macro for sysroot (function)
macro_rules! Depcrate_external_deps_rustcsysroot {
() => {
// Module: crate::external_deps::rustc
// Provides: {"sysroot"}
// Dependencies: {}
# [doc = " Query the sysroot path corresponding `rustc --print=sysroot`."] # [track_caller] pub fn sysroot () -> PathBuf { let path = rustc () . print ("sysroot") . run () . stdout_utf8 () ; PathBuf :: from_str (path . trim ()) . unwrap () }
};
}
