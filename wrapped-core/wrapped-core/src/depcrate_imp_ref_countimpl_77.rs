// Generated macro for impl_77 (impl)
macro_rules! Depcrate_imp_ref_countimpl_77 {
() => {
// Module: crate::imp::ref_count
// Provides: {"impl_77"}
// Dependencies: {}
impl RefCount { # [doc = " Creates a new `RefCount` with an initial value of `1`."] pub const fn new (count : u32) -> Self { Self (AtomicI32 :: new (count as i32)) } # [doc = " Increments the reference count, returning the new value."] pub fn add_ref (& self) -> u32 { (self . 0 . fetch_add (1 , Ordering :: Relaxed) + 1) as u32 } # [doc = " Decrements the reference count, returning the new value."] # [doc = ""] # [doc = " This operation inserts an `Acquire` fence when the reference count reaches zero."] # [doc = " This prevents reordering before the object is destroyed."] pub fn release (& self) -> u32 { let remaining = self . 0 . fetch_sub (1 , Ordering :: Release) - 1 ; match remaining . cmp (& 0) { core :: cmp :: Ordering :: Equal => fence (Ordering :: Acquire) , core :: cmp :: Ordering :: Less => panic ! ("Object has been over-released.") , core :: cmp :: Ordering :: Greater => { } } remaining as u32 } }
};
}
