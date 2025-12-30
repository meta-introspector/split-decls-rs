// Generated macro for impl_1572 (impl)
macro_rules! Depcrate_syncimpl_1572 {
() => {
// Module: crate::sync
// Provides: {"impl_1572"}
// Dependencies: {}
impl < T , A : Allocator > Arc < [mem :: MaybeUninit < T >] , A > { # [doc = " Converts to `Arc<[T]>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the inner value"] # [doc = " really is in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(get_mut_unchecked)]"] # [doc = ""] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let mut values = Arc::<[u32]>::new_uninit_slice(3);"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " let data = Arc::get_mut(&mut values).unwrap();"] # [doc = " data[0].write(1);"] # [doc = " data[1].write(2);"] # [doc = " data[2].write(3);"] # [doc = ""] # [doc = " let values = unsafe { values.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*values, [1, 2, 3])"] # [doc = " ```"] # [stable (feature = "new_uninit" , since = "1.82.0")] # [must_use = "`self` will be dropped if the result is not used"] # [inline] pub unsafe fn assume_init (self) -> Arc < [T] , A > { let (ptr , alloc) = Arc :: into_inner_with_allocator (self) ; unsafe { Arc :: from_ptr_in (ptr . as_ptr () as _ , alloc) } } }
};
}
