// Generated macro for impl_set_requires (macro)
macro_rules! Depcrate_dynamic_macrosimpl_set_requires {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_set_requires"}
// Dependencies: {}
macro_rules ! impl_set_requires { () => { # [doc = " Annotate the required input fieldset from a base type for a resolver. It"] # [doc = " is used to develop a query plan where the required fields may not be"] # [doc = " needed by the client, but the service may need additional information"] # [doc = " from other services."] # [inline] pub fn requires (self , fields : impl Into < String >) -> Self { Self { requires : Some (fields . into ()) , .. self } } } ; }
};
}
