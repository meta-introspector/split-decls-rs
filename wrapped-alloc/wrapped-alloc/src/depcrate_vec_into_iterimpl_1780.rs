// Generated macro for impl_1780 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1780 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1780"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > DoubleEndedIterator for IntoIter < T , A > { # [inline] fn next_back (& mut self) -> Option < T > { if T :: IS_ZST { if self . ptr . as_ptr () == self . end as * mut _ { return None ; } self . end = self . end . wrapping_byte_sub (1) ; Some (unsafe { ptr :: read (self . ptr . as_ptr ()) }) } else { if self . ptr == non_null ! (self . end , T) { return None ; } unsafe { self . end = self . end . sub (1) ; Some (ptr :: read (self . end)) } } } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { let step_size = self . len () . min (n) ; if T :: IS_ZST { self . end = self . end . wrapping_byte_sub (step_size) ; } else { self . end = unsafe { self . end . sub (step_size) } ; } let to_drop = ptr :: slice_from_raw_parts_mut (self . end as * mut T , step_size) ; unsafe { ptr :: drop_in_place (to_drop) ; } NonZero :: new (n - step_size) . map_or (Ok (()) , Err) } }
};
}
