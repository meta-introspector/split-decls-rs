// Generated macro for Readdir (type)
macro_rules! DepcrateReaddir {
() => {
// Module: crate
// Provides: {"Readdir"}
// Dependencies: {}
type Readdir = unsafe extern "C" fn (dirp : * mut DIR) -> * mut dirent ;
};
}
