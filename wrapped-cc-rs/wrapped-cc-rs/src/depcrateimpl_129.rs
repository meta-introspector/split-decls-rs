// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (e : io :: Error) -> Error { Error :: new (ErrorKind :: IOError , format ! ("{e}")) } }
};
}
