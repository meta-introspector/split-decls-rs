// Generated macro for impl_120 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_120 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K : Ord , V > StoreBulkMut < K , V > for ShortBoxSlice < (K , V) > { fn lm_retain < F > (& mut self , mut predicate : F) where F : FnMut (& K , & V) -> bool , { self . retain (| (k , v) | predicate (k , v)) } fn lm_extend < I > (& mut self , other : I) where I : IntoIterator < Item = (K , V) > , { let mut other = other . into_iter () ; let mut first = None ; let mut items = alloc :: vec :: Vec :: new () ; match core :: mem :: take (& mut self . 0) { ShortBoxSliceInner :: ZeroOne (zo) => { first = zo ; while let Some (next) = other . next () { if let Some (first) = first . take () { items . lm_extend ([first , next] . into_iter () . chain (other)) ; break ; } first = Some (next) ; } } ShortBoxSliceInner :: Multi (existing_items) => { items . reserve_exact (existing_items . len () + other . size_hint () . 0) ; items . extend (existing_items) ; items . lm_extend (other) ; } } if items . is_empty () { debug_assert ! (items . is_empty ()) ; self . 0 = ShortBoxSliceInner :: ZeroOne (first) ; } else { debug_assert ! (first . is_none ()) ; self . 0 = ShortBoxSliceInner :: Multi (items . into_boxed_slice ()) ; } } }
};
}
