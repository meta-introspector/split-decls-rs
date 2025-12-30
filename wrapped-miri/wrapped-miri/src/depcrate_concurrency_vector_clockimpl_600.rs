// Generated macro for impl_600 (impl)
macro_rules! Depcrate_concurrency_vector_clockimpl_600 {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"impl_600"}
// Dependencies: {}
impl Index < VectorIdx > for VClock { type Output = VTimestamp ; # [inline] fn index (& self , index : VectorIdx) -> & VTimestamp { self . as_slice () . get (index . to_u32 () . to_usize ()) . unwrap_or (& VTimestamp :: ZERO) } }
};
}
