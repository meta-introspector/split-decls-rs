// Generated macro for impl_210 (impl)
macro_rules! Depcrate_vec_into_iterimpl_210 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_210"}
// Dependencies: {}
impl < T , A : Allocator > Iterator for IntoIter < T , A > { type Item = T ; # [inline (always)] fn next (& mut self) -> Option < T > { if self . ptr == self . end { None } else if size_of :: < T > () == 0 { self . ptr = self . ptr . cast :: < u8 > () . wrapping_add (1) . cast () ; Some (unsafe { mem :: zeroed () }) } else { let old = self . ptr ; self . ptr = unsafe { self . ptr . add (1) } ; Some (unsafe { ptr :: read (old) }) } } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { let exact = if size_of :: < T > () == 0 { addr (self . end) . wrapping_sub (addr (self . ptr)) } else { unsafe { self . end . offset_from (self . ptr) as usize } } ; (exact , Some (exact)) } # [inline (always)] fn count (self) -> usize { self . len () } }
};
}
