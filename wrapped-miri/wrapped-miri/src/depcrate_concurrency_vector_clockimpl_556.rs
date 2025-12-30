// Generated macro for impl_556 (impl)
macro_rules! Depcrate_concurrency_vector_clockimpl_556 {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"impl_556"}
// Dependencies: {}
impl Index < VectorIdx > for VClock { type Output = VTimestamp ; # [inline] fn index (& self , index : VectorIdx) -> & VTimestamp { self . as_slice () . get (index . to_u32 () . to_usize ()) . unwrap_or (& VTimestamp :: ZERO) } }
};
}
