// Generated macro for impl_524 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_524 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_524"}
// Dependencies: {}
impl < T , Idx , K > Drop for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { fn drop (& mut self) { if self . maybe_changed { let val = self . pop_internal () ; unsafe { self . list . push_unchecked (val) } ; } } }
};
}
