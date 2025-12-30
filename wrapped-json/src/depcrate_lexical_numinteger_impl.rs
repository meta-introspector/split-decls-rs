// Generated macro for integer_impl (macro)
macro_rules! Depcrate_lexical_numinteger_impl {
() => {
// Module: crate::lexical::num
// Provides: {"integer_impl"}
// Dependencies: {}
macro_rules ! integer_impl { ($ ($ ty : tt) *) => { $ (impl Integer for $ ty { const ZERO : Self = 0 ; }) * } ; }
};
}
