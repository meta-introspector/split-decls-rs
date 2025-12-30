// Generated macro for canonicalize (function)
macro_rules! Depcratecanonicalize {
() => {
// Module: crate
// Provides: {"canonicalize"}
// Dependencies: {}
# [doc = " Returns the canonical, absolute form of a path with all intermediate components"] # [doc = " normalized and symbolic links resolved."] # [doc = ""] # [doc = " Wrapper for [`fs::canonicalize`](https://doc.rust-lang.org/stable/std/fs/fn.canonicalize.html)."] pub fn canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { let path = path . as_ref () ; fs :: canonicalize (path) . map_err (| source | Error :: build (source , ErrorKind :: Canonicalize , path)) }
};
}
