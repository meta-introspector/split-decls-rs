// Generated macro for impl_1745 (impl)
macro_rules! Depcrate_vec_drainimpl_1745 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_1745"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > Drop for Drain < '_ , T , A > { fn drop (& mut self) { # [doc = " Moves back the un-`Drain`ed elements to restore the original `Vec`."] struct DropGuard < 'r , 'a , T , A : Allocator > (& 'r mut Drain < 'a , T , A >) ; impl < 'r , 'a , T , A : Allocator > Drop for DropGuard < 'r , 'a , T , A > { fn drop (& mut self) { if self . 0 . tail_len > 0 { unsafe { let source_vec = self . 0 . vec . as_mut () ; let start = source_vec . len () ; let tail = self . 0 . tail_start ; if tail != start { let src = source_vec . as_ptr () . add (tail) ; let dst = source_vec . as_mut_ptr () . add (start) ; ptr :: copy (src , dst , self . 0 . tail_len) ; } source_vec . set_len (start + self . 0 . tail_len) ; } } } } let iter = mem :: take (& mut self . iter) ; let drop_len = iter . len () ; let mut vec = self . vec ; if T :: IS_ZST { unsafe { let vec = vec . as_mut () ; let old_len = vec . len () ; vec . set_len (old_len + drop_len + self . tail_len) ; vec . truncate (old_len + self . tail_len) ; } return ; } let _guard = DropGuard (self) ; if drop_len == 0 { return ; } let drop_ptr = iter . as_slice () . as_ptr () ; unsafe { let vec_ptr = vec . as_mut () . as_mut_ptr () ; let drop_offset = drop_ptr . offset_from_unsigned (vec_ptr) ; let to_drop = ptr :: slice_from_raw_parts_mut (vec_ptr . add (drop_offset) , drop_len) ; ptr :: drop_in_place (to_drop) ; } } }
};
}
