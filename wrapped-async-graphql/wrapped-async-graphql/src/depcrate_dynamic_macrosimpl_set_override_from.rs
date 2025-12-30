// Generated macro for impl_set_override_from (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_override_from {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_override_from"}
// Dependencies: {}
macro_rules ! impl_set_override_from { () => { # [doc = " Indicate that an object type's field is allowed to be resolved by"] # [doc = " multiple subgraphs"] # [inline] pub fn override_from (self , name : impl Into < String >) -> Self { Self { override_from : Some (name . into ()) , .. self } } } ; }
};
}
