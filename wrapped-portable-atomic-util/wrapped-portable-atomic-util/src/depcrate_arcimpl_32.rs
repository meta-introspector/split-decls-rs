// Generated macro for impl_32 (impl)
macro_rules! Depcrate_arcimpl_32 {
() => {
// Module: crate::arc
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < T > Arc < [mem :: MaybeUninit < T >] > { # [doc = " Converts to `Arc<[T]>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the inner value"] # [doc = " really is in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let mut values = Arc::<[u32]>::new_uninit_slice(3);"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " let data = Arc::get_mut(&mut values).unwrap();"] # [doc = " data[0].write(1);"] # [doc = " data[1].write(2);"] # [doc = " data[2].write(3);"] # [doc = ""] # [doc = " let values = unsafe { values.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*values, [1, 2, 3])"] # [doc = " ```"] # [inline] # [must_use = "`self` will be dropped if the result is not used"] pub unsafe fn assume_init (self) -> Arc < [T] > { let ptr = Arc :: into_inner_non_null (self) ; unsafe { Arc :: from_ptr (ptr . as_ptr () as * mut ArcInner < [T] >) } } }
};
}
