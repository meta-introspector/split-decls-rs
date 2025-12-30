// Generated macro for impl_1776 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1776 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1776"}
// Dependencies: {}
# [stable (feature = "vec_intoiter_as_ref" , since = "1.46.0")] impl < T , A : Allocator > AsRef < [T] > for IntoIter < T , A > { fn as_ref (& self) -> & [T] { self . as_slice () } }
};
}
