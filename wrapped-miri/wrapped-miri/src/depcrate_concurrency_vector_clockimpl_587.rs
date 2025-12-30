// Generated macro for impl_587 (impl)
macro_rules! Depcrate_concurrency_vector_clockimpl_587 {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"impl_587"}
// Dependencies: {}
impl Idx for VectorIdx { # [inline] fn new (idx : usize) -> Self { VectorIdx (u32 :: try_from (idx) . unwrap ()) } # [inline] fn index (self) -> usize { usize :: try_from (self . 0) . unwrap () } }
};
}
