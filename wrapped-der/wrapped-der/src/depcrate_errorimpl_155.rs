// Generated macro for impl_155 (impl)
macro_rules! Depcrate_errorimpl_155 {
() => {
// Module: crate::error
// Provides: {"impl_155"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < alloc :: string :: FromUtf8Error > for Error { fn from (err : alloc :: string :: FromUtf8Error) -> Error { ErrorKind :: Utf8 (err . utf8_error ()) . into () } }
};
}
