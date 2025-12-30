// Generated macro for impl_611 (impl)
macro_rules! Depcrate_errorimpl_611 {
() => {
// Module: crate::error
// Provides: {"impl_611"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { self . get_ref () . source () } }
};
}
