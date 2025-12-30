// Generated macro for impl_150 (impl)
macro_rules! Depcrate_imp_core_atomicimpl_150 {
() => {
// Module: crate::imp::core_atomic
// Provides: {"impl_150"}
// Dependencies: {}
impl < T > AtomicPtr < T > { # [inline] pub (crate) const fn new (v : * mut T) -> Self { Self { inner : core :: sync :: atomic :: AtomicPtr :: new (v) , _not_ref_unwind_safe : PhantomData } } # [inline] pub (crate) fn is_lock_free () -> bool { Self :: IS_ALWAYS_LOCK_FREE } pub (crate) const IS_ALWAYS_LOCK_FREE : bool = true ; # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub (crate) fn load (& self , order : Ordering) -> * mut T { crate :: utils :: assert_load_ordering (order) ; self . inner . load (order) } # [inline] # [cfg_attr (any (all (debug_assertions , not (portable_atomic_no_track_caller)) , miri) , track_caller)] pub (crate) fn store (& self , ptr : * mut T , order : Ordering) { crate :: utils :: assert_store_ordering (order) ; self . inner . store (ptr , order) ; } const_fn ! { const_if : # [cfg (not (portable_atomic_no_const_raw_ptr_deref))] ; # [inline] pub (crate) const fn as_ptr (& self) -> * mut * mut T { unsafe { (* (self as * const Self as * const UnsafeCell <* mut T >)) . get () } } } }
};
}
