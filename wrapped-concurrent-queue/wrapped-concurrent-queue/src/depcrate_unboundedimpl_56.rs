// Generated macro for impl_56 (impl)
macro_rules! Depcrate_unboundedimpl_56 {
() => {
// Module: crate::unbounded
// Provides: {"impl_56"}
// Dependencies: {}
impl < T > Block < T > { # [doc = " Creates an empty block."] fn new () -> Block < T > { Block { next : AtomicPtr :: new (ptr :: null_mut ()) , slots : Slot :: uninit_block () , } } # [doc = " Waits until the next pointer is set."] fn wait_next (& self) -> * mut Block < T > { loop { let next = self . next . load (Ordering :: Acquire) ; if ! next . is_null () { return next ; } busy_wait () ; } } # [doc = " Sets the `DESTROY` bit in slots starting from `start` and destroys the block."] unsafe fn destroy (this : * mut Block < T > , start : usize) { for i in start .. BLOCK_CAP - 1 { let slot = (* this) . slots . get_unchecked (i) ; if slot . state . load (Ordering :: Acquire) & READ == 0 && slot . state . fetch_or (DESTROY , Ordering :: AcqRel) & READ == 0 { return ; } } drop (Box :: from_raw (this)) ; } }
};
}
