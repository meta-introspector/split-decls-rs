// Generated macro for impl_74 (impl)
macro_rules! Depcrate_iterimpl_74 {
() => {
// Module: crate::iter
// Provides: {"impl_74"}
// Dependencies: {}
impl < T , N : ArrayLength > Iterator for GenericArrayIter < T , N > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { if self . index < self . index_back { let p = unsafe { Some (ptr :: read (self . array . get_unchecked (self . index))) } ; self . index += 1 ; p } else { None } } # [inline] fn fold < B , F > (mut self , init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let ret = unsafe { let GenericArrayIter { ref array , ref mut index , index_back , } = self ; let remaining = array . get_unchecked (* index .. index_back) ; remaining . iter () . fold (init , | acc , src | { let value = ptr :: read (src) ; * index += 1 ; f (acc , value) }) } ; mem :: forget (self) ; ret } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } # [inline (always)] fn count (self) -> usize { self . len () } fn nth (& mut self , n : usize) -> Option < T > { let next_index = self . index + cmp :: min (n , self . len ()) ; unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (self . index .. next_index)) ; } self . index = next_index ; self . next () } # [inline] fn last (mut self) -> Option < T > { self . next_back () } }
};
}
