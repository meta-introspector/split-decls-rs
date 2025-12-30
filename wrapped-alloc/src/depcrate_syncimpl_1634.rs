// Generated macro for impl_1634 (impl)
macro_rules! Depcrate_syncimpl_1634 {
() => {
// Module: crate::sync
// Provides: {"impl_1634"}
// Dependencies: {}
# [stable (feature = "boxed_slice_try_from" , since = "1.43.0")] impl < T , A : Allocator , const N : usize > TryFrom < Arc < [T] , A > > for Arc < [T ; N] , A > { type Error = Arc < [T] , A > ; fn try_from (boxed_slice : Arc < [T] , A >) -> Result < Self , Self :: Error > { if boxed_slice . len () == N { let (ptr , alloc) = Arc :: into_inner_with_allocator (boxed_slice) ; Ok (unsafe { Arc :: from_inner_in (ptr . cast () , alloc) }) } else { Err (boxed_slice) } } }
};
}
