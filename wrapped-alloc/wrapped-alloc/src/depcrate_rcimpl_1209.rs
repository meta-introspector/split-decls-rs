// Generated macro for impl_1209 (impl)
macro_rules! Depcrate_rcimpl_1209 {
() => {
// Module: crate::rc
// Provides: {"impl_1209"}
// Dependencies: {}
impl < T : ? Sized > Rc < T > { # [inline] unsafe fn from_inner (ptr : NonNull < RcInner < T > >) -> Self { unsafe { Self :: from_inner_in (ptr , Global) } } # [inline] unsafe fn from_ptr (ptr : * mut RcInner < T >) -> Self { unsafe { Self :: from_inner (NonNull :: new_unchecked (ptr)) } } }
};
}
