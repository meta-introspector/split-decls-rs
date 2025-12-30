// Generated macro for impl_586 (impl)
macro_rules! Depcrate_errorimpl_586 {
() => {
// Module: crate::error
// Provides: {"impl_586"}
// Dependencies: {}
impl From < NulError > for Error { fn from (_ : NulError) -> Error { Error :: from_str ("data contained a nul byte that could not be \
             represented as a string" ,) } }
};
}
