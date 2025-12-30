// Generated macro for impl_54 (impl)
macro_rules! Depcrate_unboundedimpl_54 {
() => {
// Module: crate::unbounded
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > Slot < T > { # [cfg (not (loom))] # [allow (clippy :: declare_interior_mutable_const)] const UNINIT : Slot < T > = Slot { value : UnsafeCell :: new (MaybeUninit :: uninit ()) , state : AtomicUsize :: new (0) , } ; # [cfg (not (loom))] fn uninit_block () -> [Slot < T > ; BLOCK_CAP] { [Self :: UNINIT ; BLOCK_CAP] } # [cfg (loom)] fn uninit_block () -> [Slot < T > ; BLOCK_CAP] { macro_rules ! repeat_31 { ($ e : expr) => { [$ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e , $ e ,] } ; } repeat_31 ! (Slot { value : UnsafeCell :: new (MaybeUninit :: uninit ()) , state : AtomicUsize :: new (0) , }) } # [doc = " Waits until a value is written into the slot."] fn wait_write (& self) { while self . state . load (Ordering :: Acquire) & WRITE == 0 { busy_wait () ; } } }
};
}
