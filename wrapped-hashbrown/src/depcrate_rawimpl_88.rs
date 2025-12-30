// Generated macro for impl_88 (impl)
macro_rules! Depcrate_rawimpl_88 {
() => {
// Module: crate::raw
// Provides: {"impl_88"}
// Dependencies: {}
impl < T : Clone , A : Allocator + Clone > RawTable < T , A > { # [doc = " Common code for `clone` and `clone_from`. Assumes:"] # [doc = " - `self.buckets() == source.buckets()`."] # [doc = " - Any existing elements have been dropped."] # [doc = " - The control bytes are not initialized yet."] # [cfg_attr (feature = "inline-more" , inline)] unsafe fn clone_from_impl (& mut self , source : & Self) { source . table . ctrl (0) . copy_to_nonoverlapping (self . table . ctrl (0) , self . table . num_ctrl_bytes ()) ; let mut guard = guard ((0 , & mut * self) , | (index , self_) | { if T :: NEEDS_DROP { for i in 0 .. * index { if self_ . is_bucket_full (i) { self_ . bucket (i) . drop () ; } } } }) ; for from in source . iter () { let index = source . bucket_index (& from) ; let to = guard . 1 . bucket (index) ; to . write (from . as_ref () . clone ()) ; guard . 0 = index + 1 ; } mem :: forget (guard) ; self . table . items = source . table . items ; self . table . growth_left = source . table . growth_left ; } }
};
}
