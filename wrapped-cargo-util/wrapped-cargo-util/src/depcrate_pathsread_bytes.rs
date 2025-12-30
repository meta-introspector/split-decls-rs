// Generated macro for read_bytes (function)
macro_rules! Depcrate_pathsread_bytes {
() => {
// Module: crate::paths
// Provides: {"read_bytes"}
// Dependencies: {}
# [doc = " Reads a file into a bytes vector."] # [doc = ""] # [doc = " Equivalent to [`std::fs::read`] with better error messages."] pub fn read_bytes (path : & Path) -> Result < Vec < u8 > > { fs :: read (path) . with_context (| | format ! ("failed to read `{}`" , path . display ())) }
};
}
