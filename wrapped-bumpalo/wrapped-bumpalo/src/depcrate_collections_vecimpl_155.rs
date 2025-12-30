// Generated macro for impl_155 (impl)
macro_rules! Depcrate_collections_vecimpl_155 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , 'bump , T , F > Iterator for DrainFilter < 'a , 'bump , T , F > where F : FnMut (& mut T) -> bool , { type Item = T ; fn next (& mut self) -> Option < T > { unsafe { while self . idx != self . old_len { let i = self . idx ; self . idx += 1 ; let v = slice :: from_raw_parts_mut (self . vec . as_mut_ptr () , self . old_len) ; if (self . pred) (& mut v [i]) { self . del += 1 ; return Some (ptr :: read (& v [i])) ; } else if self . del > 0 { let del = self . del ; let src : * const T = & v [i] ; let dst : * mut T = & mut v [i - del] ; ptr :: copy_nonoverlapping (src , dst , 1) ; } } None } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . old_len - self . idx)) } }
};
}
