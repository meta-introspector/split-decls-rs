// Generated macro for impl_set_shareable (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_shareable {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_shareable"}
// Dependencies: {}
macro_rules ! impl_set_shareable { () => { # [doc = " Indicate that an object type's field is allowed to be resolved by"] # [doc = " multiple subgraphs"] # [inline] pub fn shareable (self) -> Self { Self { shareable : true , .. self } } } ; }
};
}
