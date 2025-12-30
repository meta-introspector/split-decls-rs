// Generated macro for impl_633 (impl)
macro_rules! Depcrate_extensionsimpl_633 {
() => {
// Module: crate::extensions
// Provides: {"impl_633"}
// Dependencies: {}
impl < T : Clone + Send + Sync + 'static > AnyClone for T { fn clone_box (& self) -> Box < dyn AnyClone + Send + Sync > { Box :: new (self . clone ()) } fn as_any (& self) -> & dyn Any { self } fn as_any_mut (& mut self) -> & mut dyn Any { self } fn into_any (self : Box < Self >) -> Box < dyn Any > { self } fn type_name (& self) -> & 'static str { type_name :: < T > () } }
};
}
