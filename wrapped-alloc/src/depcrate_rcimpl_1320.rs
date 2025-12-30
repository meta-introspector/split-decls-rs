// Generated macro for impl_1320 (impl)
macro_rules! Depcrate_rcimpl_1320 {
() => {
// Module: crate::rc
// Provides: {"impl_1320"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for UniqueRc < T , A > { fn drop (& mut self) { unsafe { drop_in_place (DerefMut :: deref_mut (self)) ; self . ptr . as_ref () . dec_weak () ; if self . ptr . as_ref () . weak () == 0 { self . alloc . deallocate (self . ptr . cast () , Layout :: for_value_raw (self . ptr . as_ptr ())) ; } } } }
};
}
