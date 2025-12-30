// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_vec_deque_drainDrain {
() => {
// Module: crate::collections::vec_deque::drain
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of a `VecDeque`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`VecDeque`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: VecDeque::drain"] # [stable (feature = "drain" , since = "1.6.0")] pub struct Drain < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { deque : NonNull < VecDeque < T , A > > , drain_len : usize , idx : usize , new_len : usize , remaining : usize , _marker : PhantomData < & 'a T > , }
};
}
