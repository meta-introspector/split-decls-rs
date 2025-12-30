// Generated macro for impl_1740 (impl)
macro_rules! Depcrate_vec_drainimpl_1740 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_1740"}
// Dependencies: {}
# [stable (feature = "vec_drain_as_slice" , since = "1.46.0")] impl < 'a , T , A : Allocator > AsRef < [T] > for Drain < 'a , T , A > { fn as_ref (& self) -> & [T] { self . as_slice () } }
};
}
