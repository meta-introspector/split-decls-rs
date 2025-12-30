// Generated macro for impl_set_provides (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_provides {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_provides"}
// Dependencies: {}
macro_rules ! impl_set_provides { () => { # [doc = " Annotate the expected returned fieldset from a field on a base type that"] # [doc = " is guaranteed to be selectable by the gateway."] # [inline] pub fn provides (self , fields : impl Into < String >) -> Self { Self { provides : Some (fields . into ()) , .. self } } } ; }
};
}
