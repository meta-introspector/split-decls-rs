// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl GlobError { # [doc = " The Path that the error corresponds to."] pub fn path (& self) -> & Path { & self . path } # [doc = " The error in question."] pub fn error (& self) -> & io :: Error { & self . error } # [doc = " Consumes self, returning the _raw_ underlying `io::Error`"] # [deprecated (note = "use `.into` instead")] pub fn into_error (self) -> io :: Error { self . error } }
};
}
