// Generated macro for impl_set_description (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_description {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_description"}
// Dependencies: {}
macro_rules ! impl_set_description { () => { # [doc = " Set the description"] # [inline] pub fn description (self , description : impl Into < String >) -> Self { Self { description : Some (description . into ()) , .. self } } } ; }
};
}
