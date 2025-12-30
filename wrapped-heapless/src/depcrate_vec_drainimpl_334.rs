// Generated macro for impl_334 (impl)
macro_rules! Depcrate_vec_drainimpl_334 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_334"}
// Dependencies: {}
impl < T , LenT : LenType > Drop for Drain < '_ , T , LenT > { fn drop (& mut self) { # [doc = " Moves back the un-`Drain`ed elements to restore the original `Vec`."] struct DropGuard < 'r , 'a , T , LenT : LenType > (& 'r mut Drain < 'a , T , LenT >) ; impl < T , LenT : LenType > Drop for DropGuard < '_ , '_ , T , LenT > { fn drop (& mut self) { if self . 0 . tail_len > LenT :: ZERO { unsafe { let source_vec = self . 0 . vec . as_mut () ; let start = source_vec . len () ; let tail = self . 0 . tail_start . into_usize () ; let tail_len = self . 0 . tail_len . into_usize () ; if tail != start { let dst = source_vec . as_mut_ptr () . add (start) ; let src = source_vec . as_ptr () . add (tail) ; ptr :: copy (src , dst , tail_len) ; } source_vec . set_len (start + tail_len) ; } } } } let iter = mem :: take (& mut self . iter) ; let drop_len = iter . len () ; let mut vec = self . vec ; if size_of :: < T > () == 0 { unsafe { let vec = vec . as_mut () ; let old_len = vec . len () ; let tail_len = self . tail_len . into_usize () ; vec . set_len (old_len + drop_len + tail_len) ; vec . truncate (old_len + tail_len) ; } return ; } let _guard = DropGuard (self) ; if drop_len == 0 { return ; } let drop_ptr = iter . as_slice () . as_ptr () ; unsafe { let vec_ptr = vec . as_mut () . as_mut_ptr () ; let drop_offset = (drop_ptr as usize - vec_ptr as usize) / size_of :: < T > () ; let to_drop = ptr :: slice_from_raw_parts_mut (vec_ptr . add (drop_offset) , drop_len) ; ptr :: drop_in_place (to_drop) ; } } }
};
}
