// Generated macro for impl_1790 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1790 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1790"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for IntoIter < T , A > { fn drop (& mut self) { struct DropGuard < 'a , T , A : Allocator > (& 'a mut IntoIter < T , A >) ; impl < T , A : Allocator > Drop for DropGuard < '_ , T , A > { fn drop (& mut self) { unsafe { let alloc = ManuallyDrop :: take (& mut self . 0 . alloc) ; let _ = RawVec :: from_nonnull_in (self . 0 . buf , self . 0 . cap , alloc) ; } } } let guard = DropGuard (self) ; unsafe { ptr :: drop_in_place (guard . 0 . as_raw_mut_slice ()) ; } } }
};
}
