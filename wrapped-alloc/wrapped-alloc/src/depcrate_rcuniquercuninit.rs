// Generated macro for UniqueRcUninit (struct)
macro_rules! Depcrate_rcUniqueRcUninit {
() => {
// Module: crate::rc
// Provides: {"UniqueRcUninit"}
// Dependencies: {}
# [doc = " A unique owning pointer to a [`RcInner`] **that does not imply the contents are initialized,**"] # [doc = " but will deallocate it (without dropping the value) when dropped."] # [doc = ""] # [doc = " This is a helper for [`Rc::make_mut()`] to ensure correct cleanup on panic."] # [doc = " It is nearly a duplicate of `UniqueRc<MaybeUninit<T>, A>` except that it allows `T: !Sized`,"] # [doc = " which `MaybeUninit` does not."] # [cfg (not (no_global_oom_handling))] struct UniqueRcUninit < T : ? Sized , A : Allocator > { ptr : NonNull < RcInner < T > > , layout_for_value : Layout , alloc : Option < A > , }
};
}
