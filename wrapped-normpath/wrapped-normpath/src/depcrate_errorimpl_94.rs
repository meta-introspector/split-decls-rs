// Generated macro for impl_94 (impl)
macro_rules! Depcrate_errorimpl_94 {
() => {
// Module: crate::error
// Provides: {"impl_94"}
// Dependencies: {}
impl MissingPrefixBufError { # [doc = " Returns a reference to the path that caused this error."] # [inline] # [must_use] pub fn as_path (& self) -> & Path { & self . 0 } # [doc = " Returns the path that caused this error."] # [inline] # [must_use] pub fn into_path_buf (self) -> PathBuf { self . 0 } }
};
}
