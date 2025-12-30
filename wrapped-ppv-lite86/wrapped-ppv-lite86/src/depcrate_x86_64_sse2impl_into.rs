// Generated macro for impl_into (macro)
macro_rules! Depcrate_x86_64_sse2impl_into {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_into"}
// Dependencies: {}
macro_rules ! impl_into { ($ from : ident , $ to : ident) => { impl < S3 , S4 , NI > From <$ from < S3 , S4 , NI >> for $ to < S3 , S4 , NI > { # [inline (always)] fn from (x : $ from < S3 , S4 , NI >) -> Self { $ to :: new (x . x) } } } ; }
};
}
