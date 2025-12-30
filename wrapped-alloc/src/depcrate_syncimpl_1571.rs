// Generated macro for impl_1571 (impl)
macro_rules! Depcrate_syncimpl_1571 {
() => {
// Module: crate::sync
// Provides: {"impl_1571"}
// Dependencies: {}
impl < T , A : Allocator > Arc < mem :: MaybeUninit < T > , A > { # [doc = " Converts to `Arc<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the inner value"] # [doc = " really is in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(get_mut_unchecked)]"] # [doc = ""] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let mut five = Arc::<u32>::new_uninit();"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " Arc::get_mut(&mut five).unwrap().write(5);"] # [doc = ""] # [doc = " let five = unsafe { five.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*five, 5)"] # [doc = " ```"] # [stable (feature = "new_uninit" , since = "1.82.0")] # [must_use = "`self` will be dropped if the result is not used"] # [inline] pub unsafe fn assume_init (self) -> Arc < T , A > { let (ptr , alloc) = Arc :: into_inner_with_allocator (self) ; unsafe { Arc :: from_inner_in (ptr . cast () , alloc) } } }
};
}
