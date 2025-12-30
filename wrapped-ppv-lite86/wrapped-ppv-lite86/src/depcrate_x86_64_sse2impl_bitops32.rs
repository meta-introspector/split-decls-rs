// Generated macro for impl_bitops32 (macro)
macro_rules! Depcrate_x86_64_sse2impl_bitops32 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_bitops32"}
// Dependencies: {}
macro_rules ! impl_bitops32 { ($ vec : ident) => { impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps32 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord32 { } } ; }
};
}
