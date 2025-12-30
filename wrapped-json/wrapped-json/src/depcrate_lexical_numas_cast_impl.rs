// Generated macro for as_cast_impl (macro)
macro_rules! Depcrate_lexical_numas_cast_impl {
() => {
// Module: crate::lexical::num
// Provides: {"as_cast_impl"}
// Dependencies: {}
macro_rules ! as_cast_impl { ($ ty : ident , $ method : ident) => { impl AsCast for $ ty { # [inline] fn as_cast < N : AsPrimitive > (n : N) -> Self { n .$ method () } } } ; }
};
}
