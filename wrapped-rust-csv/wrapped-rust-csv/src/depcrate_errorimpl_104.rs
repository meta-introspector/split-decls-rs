// Generated macro for impl_104 (impl)
macro_rules! Depcrate_errorimpl_104 {
() => {
// Module: crate::error
// Provides: {"impl_104"}
// Dependencies: {}
impl < W > IntoInnerError < W > { # [doc = " Creates a new `IntoInnerError`."] # [doc = ""] # [doc = " (This is a visibility hack. It's public in this module, but not in the"] # [doc = " crate.)"] pub (crate) fn new (wtr : W , err : io :: Error) -> IntoInnerError < W > { IntoInnerError { wtr , err } } # [doc = " Returns the error which caused the call to `into_inner` to fail."] # [doc = ""] # [doc = " This error was returned when attempting to flush the internal buffer."] pub fn error (& self) -> & io :: Error { & self . err } # [doc = " Consumes the [`IntoInnerError`] and returns the error which caused the"] # [doc = " call to [`Writer::into_inner`](crate::Writer::into_inner) to fail."] # [doc = ""] # [doc = " Unlike [`IntoInnerError::error`], this can be used to obtain ownership"] # [doc = " of the underlying error."] pub fn into_error (self) -> io :: Error { self . err } # [doc = " Returns the underlying writer which generated the error."] # [doc = ""] # [doc = " The returned value can be used for error recovery, such as"] # [doc = " re-inspecting the buffer."] pub fn into_inner (self) -> W { self . wtr } }
};
}
