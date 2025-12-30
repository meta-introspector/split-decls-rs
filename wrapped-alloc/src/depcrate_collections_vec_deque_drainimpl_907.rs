// Generated macro for impl_907 (impl)
macro_rules! Depcrate_collections_vec_deque_drainimpl_907 {
() => {
// Module: crate::collections::vec_deque::drain
// Provides: {"impl_907"}
// Dependencies: {}
impl < 'a , T , A : Allocator > Drain < 'a , T , A > { pub (super) unsafe fn new (deque : & 'a mut VecDeque < T , A > , drain_start : usize , drain_len : usize ,) -> Self { let orig_len = mem :: replace (& mut deque . len , drain_start) ; let new_len = orig_len - drain_len ; Drain { deque : NonNull :: from (deque) , drain_len , idx : drain_start , new_len , remaining : drain_len , _marker : PhantomData , } } unsafe fn as_slices (& self) -> (* mut [T] , * mut [T]) { unsafe { let deque = self . deque . as_ref () ; let logical_remaining_range = self . idx .. self . idx + self . remaining ; let (a_range , b_range) = deque . slice_ranges (logical_remaining_range . clone () , logical_remaining_range . end) ; (deque . buffer_range (a_range) , deque . buffer_range (b_range)) } } }
};
}
