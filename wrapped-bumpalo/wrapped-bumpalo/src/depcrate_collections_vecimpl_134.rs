// Generated macro for impl_134 (impl)
macro_rules! Depcrate_collections_vecimpl_134 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'bump , T : 'bump > Iterator for IntoIter < 'bump , T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { unsafe { if self . ptr as * const _ == self . end { None } else if mem :: size_of :: < T > () == 0 { self . ptr = arith_offset (self . ptr as * const i8 , 1) as * mut T ; Some (mem :: zeroed ()) } else { let old = self . ptr ; self . ptr = self . ptr . offset (1) ; Some (ptr :: read (old)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let exact = if mem :: size_of :: < T > () == 0 { (self . end as usize) . wrapping_sub (self . ptr as usize) } else { unsafe { offset_from (self . end , self . ptr) as usize } } ; (exact , Some (exact)) } # [inline] fn count (self) -> usize { self . len () } }
};
}
