// Generated macro for impl_130 (impl)
macro_rules! Depcrate_rwlockimpl_130 {
() => {
// Module: crate::rwlock
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > RwLockReadGuardArc < T > { # [doc = " Constructs the underlying `Arc` back from the underlying `RwLock`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Both the returned `Arc` and the guard will decrement their reference"] # [doc = " counts on drop! So one of the two must be forgotten."] # [inline] unsafe fn inner_arc (guard : & Self) -> ManuallyDrop < Arc < RwLock < T > > > { ManuallyDrop :: new (Arc :: from_raw (guard . lock . as_ptr () . cast ())) } # [doc = " Constructs a guard from the underlying `Arc`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " A read lock must be acquired before calling this."] # [inline] unsafe fn from_arc (arc : Arc < RwLock < T > >) -> Self { let ptr = Arc :: into_raw (arc) ; Self { lock : NonNull :: new (ptr as * mut RwLock < T > as * mut T) . unwrap () , } } }
};
}
