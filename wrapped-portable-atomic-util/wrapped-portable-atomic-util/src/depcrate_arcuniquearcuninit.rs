// Generated macro for UniqueArcUninit (struct)
macro_rules! Depcrate_arcUniqueArcUninit {
() => {
// Module: crate::arc
// Provides: {"UniqueArcUninit"}
// Dependencies: {}
# [doc = " A unique owning pointer to an [`ArcInner`] **that does not imply the contents are initialized,**"] # [doc = " but will deallocate it (without dropping the value) when dropped."] # [doc = ""] # [doc = " This is a helper for [`Arc::make_mut()`] to ensure correct cleanup on panic."] struct UniqueArcUninit < T : ? Sized > { ptr : NonNull < ArcInner < T > > , layout_for_value : Layout , }
};
}
