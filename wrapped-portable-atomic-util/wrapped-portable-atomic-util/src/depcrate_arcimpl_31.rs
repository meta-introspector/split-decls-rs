// Generated macro for impl_31 (impl)
macro_rules! Depcrate_arcimpl_31 {
() => {
// Module: crate::arc
// Provides: {"impl_31"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_maybe_uninit))] impl < T > Arc < mem :: MaybeUninit < T > > { # [doc = " Converts to `Arc<T>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " As with [`MaybeUninit::assume_init`],"] # [doc = " it is up to the caller to guarantee that the inner value"] # [doc = " really is in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized"] # [doc = " causes immediate undefined behavior."] # [doc = ""] # [doc = " [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let mut five = Arc::<u32>::new_uninit();"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " Arc::get_mut(&mut five).unwrap().write(5);"] # [doc = ""] # [doc = " let five = unsafe { five.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*five, 5)"] # [doc = " ```"] # [inline] # [must_use = "`self` will be dropped if the result is not used"] pub unsafe fn assume_init (self) -> Arc < T > { let ptr = Arc :: into_inner_non_null (self) ; unsafe { Arc :: from_inner (ptr . cast :: < ArcInner < T > > ()) } } }
};
}
