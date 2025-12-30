// Generated macro for impl_1337 (impl)
macro_rules! Depcrate_pathimpl_1337 {
() => {
// Module: crate::path
// Provides: {"impl_1337"}
// Dependencies: {}
impl PathParams { # [doc = " Gets the number of components in the path."] pub fn components (& self) -> SizeRange { self . components . clone () } # [doc = " Sets the number of components in the path."] pub fn with_components (mut self , components : impl Into < SizeRange >) -> Self { self . components = components . into () ; self } # [doc = " Gets the regular expression to generate individual components."] pub fn component_regex (& self) -> StringParam { self . component_regex } # [doc = " Sets the regular expression to generate individual components."] pub fn with_component_regex (mut self , component_regex : impl Into < StringParam > ,) -> Self { self . component_regex = component_regex . into () ; self } }
};
}
