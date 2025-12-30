// Generated macro for impl_97 (impl)
macro_rules! Depcrate_errorimpl_97 {
() => {
// Module: crate::error
// Provides: {"impl_97"}
// Dependencies: {}
impl StdError for FromUtf8Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . err) } }
};
}
