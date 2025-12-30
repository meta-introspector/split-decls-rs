// Generated macro for UniqueArcUninit (struct)
macro_rules! Depcrate_syncUniqueArcUninit {
() => {
// Module: crate::sync
// Provides: {"UniqueArcUninit"}
// Dependencies: {}
# [doc = " A unique owning pointer to an [`ArcInner`] **that does not imply the contents are initialized,**"] # [doc = " but will deallocate it (without dropping the value) when dropped."] # [doc = ""] # [doc = " This is a helper for [`Arc::make_mut()`] to ensure correct cleanup on panic."] # [cfg (not (no_global_oom_handling))] struct UniqueArcUninit < T : ? Sized , A : Allocator > { ptr : NonNull < ArcInner < T > > , layout_for_value : Layout , alloc : Option < A > , }
};
}
