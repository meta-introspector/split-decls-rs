// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl Error { # [doc = " Return the glob that caused this error, if one exists."] pub fn glob (& self) -> Option < & str > { self . glob . as_ref () . map (| s | & * * s) } # [doc = " Return the kind of this error."] pub fn kind (& self) -> & ErrorKind { & self . kind } }
};
}
