// Generated macro for impl_118 (impl)
macro_rules! Depcrate_boxedimpl_118 {
() => {
// Module: crate::boxed
// Provides: {"impl_118"}
// Dependencies: {}
impl < T , A : Allocator > Box < [mem :: MaybeUninit < T >] , A > { # [doc = " Converts to `Box<[T], A>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the values"] # [doc = " really are in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut values = Box::<[u32]>::new_uninit_slice(3);"] # [doc = " // Deferred initialization:"] # [doc = " values[0].write(1);"] # [doc = " values[1].write(2);"] # [doc = " values[2].write(3);"] # [doc = " let values = unsafe { values.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*values, [1, 2, 3])"] # [doc = " ```"] # [stable (feature = "new_uninit" , since = "1.82.0")] # [inline] pub unsafe fn assume_init (self) -> Box < [T] , A > { let (raw , alloc) = Box :: into_raw_with_allocator (self) ; unsafe { Box :: from_raw_in (raw as * mut [T] , alloc) } } }
};
}
