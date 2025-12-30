// Generated macro for impl_1713 (impl)
macro_rules! Depcrate_vec_extract_ifimpl_1713 {
() => {
// Module: crate::vec::extract_if
// Provides: {"impl_1713"}
// Dependencies: {}
# [stable (feature = "extract_if" , since = "1.87.0")] impl < T , F , A : Allocator > Iterator for ExtractIf < '_ , T , F , A > where F : FnMut (& mut T) -> bool , { type Item = T ; fn next (& mut self) -> Option < T > { unsafe { while self . idx < self . end { let i = self . idx ; let v = slice :: from_raw_parts_mut (self . vec . as_mut_ptr () , self . old_len) ; let drained = (self . pred) (& mut v [i]) ; self . idx += 1 ; if drained { self . del += 1 ; return Some (ptr :: read (& v [i])) ; } else if self . del > 0 { let del = self . del ; let src : * const T = & v [i] ; let dst : * mut T = & mut v [i - del] ; ptr :: copy_nonoverlapping (src , dst , 1) ; } } None } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . end - self . idx)) } }
};
}
