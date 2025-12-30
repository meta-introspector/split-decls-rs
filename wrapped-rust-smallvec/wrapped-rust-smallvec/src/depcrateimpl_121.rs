// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'a , T : 'a , const N : usize > Drop for Drain < 'a , T , N > { fn drop (& mut self) { # [doc = " Moves back the un-`Drain`ed elements to restore the original `Vec`."] struct DropGuard < 'r , 'a , T , const N : usize > (& 'r mut Drain < 'a , T , N >) ; impl < 'r , 'a , T , const N : usize > Drop for DropGuard < 'r , 'a , T , N > { fn drop (& mut self) { if self . 0 . tail_len > 0 { unsafe { let source_vec = self . 0 . vec . as_mut () ; let start = source_vec . len () ; let tail = self . 0 . tail_start ; if tail != start { let ptr = source_vec . as_mut_ptr () ; let src = ptr . add (tail) ; let dst = ptr . add (start) ; core :: ptr :: copy (src , dst , self . 0 . tail_len) ; } source_vec . set_len (start + self . 0 . tail_len) ; } } } } let iter = core :: mem :: take (& mut self . iter) ; let drop_len = iter . len () ; let mut vec = self . vec ; if SmallVec :: < T , N > :: is_zst () { unsafe { let vec = vec . as_mut () ; let old_len = vec . len () ; vec . set_len (old_len + drop_len + self . tail_len) ; vec . truncate (old_len + self . tail_len) ; } return ; } let _guard = DropGuard (self) ; if drop_len == 0 { return ; } let drop_ptr = iter . as_slice () . as_ptr () ; unsafe { let vec_ptr = vec . as_mut () . as_mut_ptr () ; let drop_offset = drop_ptr . offset_from (vec_ptr) as usize ; let to_drop = core :: ptr :: slice_from_raw_parts_mut (vec_ptr . add (drop_offset) , drop_len) ; core :: ptr :: drop_in_place (to_drop) ; } } }
};
}
