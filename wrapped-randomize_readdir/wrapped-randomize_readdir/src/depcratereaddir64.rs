// Generated macro for readdir64 (function)
macro_rules! Depcratereaddir64 {
() => {
// Module: crate
// Provides: {"readdir64"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn readdir64 (dirp : * mut DIR) -> * mut dirent64 { STATE . wait () . wrapped_readdir64 (dirp) }
};
}
