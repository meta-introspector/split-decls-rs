// Generated macro for impl_85 (impl)
macro_rules! Depcrate_arcimpl_85 {
() => {
// Module: crate::arc
// Provides: {"impl_85"}
// Dependencies: {}
impl < T : ? Sized > Drop for UniqueArcUninit < T > { fn drop (& mut self) { unsafe { Global . deallocate (self . ptr . cast :: < u8 > () , arc_inner_layout_for_value_layout (self . layout_for_value) ,) ; } } }
};
}
