// Generated macro for impl_45 (impl)
macro_rules! Depcrate_arcimpl_45 {
() => {
// Module: crate::arc
// Provides: {"impl_45"}
// Dependencies: {}
impl < T > Weak < T > { # [doc = " Constructs a new `Weak<T>`, without allocating any memory."] # [doc = " Calling [`upgrade`] on the return value always gives [`None`]."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Weak;"] # [doc = ""] # [doc = " let empty: Weak<i64> = Weak::new();"] # [doc = " assert!(empty.upgrade().is_none());"] # [doc = " ```"] # [inline] # [must_use] pub const fn new () -> Self { Self { ptr : unsafe { NonNull :: new_unchecked (strict :: without_provenance_mut :: < ArcInner < T > > (usize :: MAX)) } , } } # [inline] # [must_use] fn new_uninit_ptr () -> NonNull < ArcInner < T > > { unsafe { NonNull :: new_unchecked (Self :: allocate_for_layout (Layout :: new :: < T > () , | layout | Global . allocate (layout) , | ptr | ptr as * mut _ ,)) } } }
};
}
