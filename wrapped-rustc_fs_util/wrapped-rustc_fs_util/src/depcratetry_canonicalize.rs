// Generated macro for try_canonicalize (function)
macro_rules! Depcratetry_canonicalize {
() => {
// Module: crate
// Provides: {"try_canonicalize"}
// Dependencies: {}
# [inline] pub fn try_canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { fs :: canonicalize (& path) . or_else (| _ | absolute (& path)) }
};
}
