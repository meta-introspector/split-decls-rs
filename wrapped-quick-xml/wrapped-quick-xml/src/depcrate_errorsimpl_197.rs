// Generated macro for impl_197 (impl)
macro_rules! Depcrate_errorsimpl_197 {
() => {
// Module: crate::errors
// Provides: {"impl_197"}
// Dependencies: {}
impl From < IoError > for Error { # [doc = " Creates a new `Error::Io` from the given error"] # [inline] fn from (error : IoError) -> Error { Self :: Io (Arc :: new (error)) } }
};
}
