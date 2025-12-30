// Generated macro for impl_1674 (impl)
macro_rules! Depcrate_syncimpl_1674 {
() => {
// Module: crate::sync
// Provides: {"impl_1674"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for UniqueArc < T , A > { fn drop (& mut self) { let _weak = Weak { ptr : self . ptr , alloc : & self . alloc } ; unsafe { ptr :: drop_in_place (& mut (* self . ptr . as_ptr ()) . data) } ; } }
};
}
