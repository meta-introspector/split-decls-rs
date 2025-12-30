// Generated macro for impl_set_external (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_external {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_external"}
// Dependencies: {}
macro_rules ! impl_set_external { () => { # [doc = " Mark a field as owned by another service. This allows service A to use"] # [doc = " fields from service B while also knowing at runtime the types of that"] # [doc = " field."] # [inline] pub fn external (self) -> Self { Self { external : true , .. self } } } ; }
};
}
