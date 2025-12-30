// Generated macro for canonicalize (function)
macro_rules! Depcratecanonicalize {
() => {
// Module: crate
// Provides: {"canonicalize"}
// Dependencies: {}
# [doc = " Like `std::fs::canonicalize()`, but on Windows it outputs the most"] # [doc = " compatible form of a path instead of UNC."] # [inline (always)] # [doc (alias = "realpath")] pub fn canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { let path = path . as_ref () ; # [cfg (not (windows))] { fs :: canonicalize (path) } # [cfg (windows)] { canonicalize_win (path) } }
};
}
