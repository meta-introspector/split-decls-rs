// Generated macro for impl_1727 (impl)
macro_rules! Depcrate_vec_spliceimpl_1727 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_1727"}
// Dependencies: {}
# [doc = " Private helper methods for `Splice::drop`"] impl < T , A : Allocator > Drain < '_ , T , A > { # [doc = " The range from `self.vec.len` to `self.tail_start` contains elements"] # [doc = " that have been moved out."] # [doc = " Fill that range as much as possible with new elements from the `replace_with` iterator."] # [doc = " Returns `true` if we filled the entire range. (`replace_with.next()` didn’t return `None`.)"] unsafe fn fill < I : Iterator < Item = T > > (& mut self , replace_with : & mut I) -> bool { let vec = unsafe { self . vec . as_mut () } ; let range_start = vec . len ; let range_end = self . tail_start ; let range_slice = unsafe { slice :: from_raw_parts_mut (vec . as_mut_ptr () . add (range_start) , range_end - range_start) } ; for place in range_slice { if let Some (new_item) = replace_with . next () { unsafe { ptr :: write (place , new_item) } ; vec . len += 1 ; } else { return false ; } } true } # [doc = " Makes room for inserting more elements before the tail."] # [track_caller] unsafe fn move_tail (& mut self , additional : usize) { let vec = unsafe { self . vec . as_mut () } ; let len = self . tail_start + self . tail_len ; vec . buf . reserve (len , additional) ; let new_tail_start = self . tail_start + additional ; unsafe { let src = vec . as_ptr () . add (self . tail_start) ; let dst = vec . as_mut_ptr () . add (new_tail_start) ; ptr :: copy (src , dst , self . tail_len) ; } self . tail_start = new_tail_start ; } }
};
}
