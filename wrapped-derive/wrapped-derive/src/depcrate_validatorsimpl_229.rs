// Generated macro for impl_229 (impl)
macro_rules! Depcrate_validatorsimpl_229 {
() => {
// Module: crate::validators
// Provides: {"impl_229"}
// Dependencies: {}
impl ToTokens for Number { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Number :: F64 (n) => tokens . extend (quote ! (# n as f64)) , Number :: I64 (n) => tokens . extend (quote ! (# n as i64)) , } } }
};
}
