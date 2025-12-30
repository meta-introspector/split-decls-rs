// Generated macro for impl_bitops64 (macro)
macro_rules! Depcrate_x86_64_sse2impl_bitops64 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_bitops64"}
// Dependencies: {}
macro_rules ! impl_bitops64 { ($ vec : ident) => { impl_bitops32 ! ($ vec) ; impl < S3 : Copy , S4 : Copy , NI : Copy > BitOps64 for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: RotateEachWord64 + RotateEachWord32 { } } ; }
};
}
