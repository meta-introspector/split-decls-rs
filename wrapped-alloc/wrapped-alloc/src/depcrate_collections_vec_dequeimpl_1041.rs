// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1041 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1041"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for VecDeque < T , A > { fn drop (& mut self) { # [doc = " Runs the destructor for all items in the slice when it gets dropped (normally or"] # [doc = " during unwinding)."] struct Dropper < 'a , T > (& 'a mut [T]) ; impl < 'a , T > Drop for Dropper < 'a , T > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . 0) ; } } } let (front , back) = self . as_mut_slices () ; unsafe { let _back_dropper = Dropper (back) ; ptr :: drop_in_place (front) ; } } }
};
}
