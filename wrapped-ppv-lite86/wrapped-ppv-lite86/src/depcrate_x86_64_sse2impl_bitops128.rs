// Generated macro for impl_bitops128 (macro)
macro_rules! Depcrate_x86_64_sse2impl_bitops128 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_bitops128"}
// Dependencies: {}
macro_rules ! impl_bitops128 { ($ vec : ident) => { impl_bitops64 ! ($ vec) ; impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps128 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord128 { } } ; }
};
}
