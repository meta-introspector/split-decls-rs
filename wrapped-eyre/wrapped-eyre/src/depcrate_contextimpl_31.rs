// Generated macro for impl_31 (impl)
macro_rules! Depcrate_contextimpl_31 {
() => {
// Module: crate::context
// Provides: {"impl_31"}
// Dependencies: {}
impl < D > StdError for ContextError < D , Report > where D : Display , { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (ErrorImpl :: error (self . error . inner . as_ref ())) } }
};
}
