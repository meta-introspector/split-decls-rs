// Generated macro for impl_1533 (impl)
macro_rules! Depcrate_slice_sortimpl_1533 {
() => {
// Module: crate::slice::sort
// Provides: {"impl_1533"}
// Dependencies: {}
impl < T > Drop for MergeHole < T > { fn drop (& mut self) { unsafe { let len = self . end . offset_from (self . start) as usize ; ptr :: copy_nonoverlapping (self . start , self . dest , len) ; } } }
};
}
