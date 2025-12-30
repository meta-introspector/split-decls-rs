// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_rcimpl_1267 {
() => {
// Module: crate::rc
// Provides: {"impl_1267"}
// Dependencies: {}
# [stable (feature = "boxed_slice_try_from" , since = "1.43.0")] impl < T , A : Allocator , const N : usize > TryFrom < Rc < [T] , A > > for Rc < [T ; N] , A > { type Error = Rc < [T] , A > ; fn try_from (boxed_slice : Rc < [T] , A >) -> Result < Self , Self :: Error > { if boxed_slice . len () == N { let (ptr , alloc) = Rc :: into_inner_with_allocator (boxed_slice) ; Ok (unsafe { Rc :: from_inner_in (ptr . cast () , alloc) }) } else { Err (boxed_slice) } } }
};
}
