// Generated macro for impl_95 (impl)
macro_rules! Depcrate_errorimpl_95 {
() => {
// Module: crate::error
// Provides: {"impl_95"}
// Dependencies: {}
impl StdError for RenderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (self . reason ()) } }
};
}
