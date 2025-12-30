// Generated macro for impl_60 (impl)
macro_rules! Depcrate_boxedimpl_60 {
() => {
// Module: crate::boxed
// Provides: {"impl_60"}
// Dependencies: {}
impl < T , A : Allocator > Box < [mem :: MaybeUninit < T >] , A > { # [doc = " Converts to `Box<[T], A>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the values"] # [doc = " really are in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " let mut values = Box::<[u32]>::new_uninit_slice(3);"] # [doc = ""] # [doc = " let values = unsafe {"] # [doc = "     // Deferred initialization:"] # [doc = "     values[0].as_mut_ptr().write(1);"] # [doc = "     values[1].as_mut_ptr().write(2);"] # [doc = "     values[2].as_mut_ptr().write(3);"] # [doc = ""] # [doc = "     values.assume_init()"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(*values, [1, 2, 3])"] # [doc = " ```"] # [inline (always)] pub unsafe fn assume_init (self) -> Box < [T] , A > { let (raw , alloc) = Self :: into_raw_with_allocator (self) ; unsafe { Box :: < [T] , A > :: from_raw_in (raw as * mut [T] , alloc) } } }
};
}
