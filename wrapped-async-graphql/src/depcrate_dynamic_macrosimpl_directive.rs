// Generated macro for impl_directive (macro)
macro_rules! Depcrate_dynamic_macrosimpl_directive {
() => {
// Module: crate::dynamic::macros
// Provides: {"impl_directive"}
// Dependencies: {}
macro_rules ! impl_directive { () => { # [doc = " Attach directive to the entity"] # [inline] pub fn directive (mut self , directive : Directive) -> Self { self . directives . push (directive) ; self } } ; }
};
}
