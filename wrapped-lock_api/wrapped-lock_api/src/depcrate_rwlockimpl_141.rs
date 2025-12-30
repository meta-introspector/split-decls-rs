// Generated macro for impl_141 (impl)
macro_rules! Depcrate_rwlockimpl_141 {
() => {
// Module: crate::rwlock
// Provides: {"impl_141"}
// Dependencies: {}
impl < R : RawRwLock , T > RwLock < R , T > { # [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [inline] pub const fn new (val : T) -> RwLock < R , T > { RwLock { data : UnsafeCell :: new (val) , raw : R :: INIT , } } # [doc = " Consumes this `RwLock`, returning the underlying data."] # [inline] # [allow (unused_unsafe)] pub fn into_inner (self) -> T { unsafe { self . data . into_inner () } } }
};
}
