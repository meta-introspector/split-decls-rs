// Generated macro for impl_1023 (impl)
macro_rules! Depcrate_runtime_defineimpl_1023 {
() => {
// Module: crate::runtime::define
// Provides: {"impl_1023"}
// Dependencies: {}
impl < T > Log2Alignment for T { const LOG2_ALIGNMENT : u8 = { let align = mem :: align_of :: < T > () ; assert ! (align . count_ones () == 1 , "alignment required to be a power of 2") ; align . trailing_zeros () as u8 } ; }
};
}
