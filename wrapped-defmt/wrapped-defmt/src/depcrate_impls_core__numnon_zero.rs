// Generated macro for non_zero (macro)
macro_rules! Depcrate_impls_core__numnon_zero {
() => {
// Module: crate::impls::core_::num
// Provides: {"non_zero"}
// Dependencies: {}
macro_rules ! non_zero { ($ type : ty , $ hint : literal) => { impl Format for $ type { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , $ hint , self . get ()) ; } } } ; }
};
}
