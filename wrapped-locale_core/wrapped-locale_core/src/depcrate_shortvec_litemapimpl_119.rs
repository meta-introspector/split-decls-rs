// Generated macro for impl_119 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_119 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < K , V > StoreMut < K , V > for ShortBoxSlice < (K , V) > { fn lm_with_capacity (_capacity : usize) -> Self { ShortBoxSlice :: new () } fn lm_reserve (& mut self , _additional : usize) { } fn lm_get_mut (& mut self , index : usize) -> Option < (& K , & mut V) > { self . get_mut (index) . map (| elt | (& elt . 0 , & mut elt . 1)) } fn lm_push (& mut self , key : K , value : V) { self . push ((key , value)) } fn lm_insert (& mut self , index : usize , key : K , value : V) { self . insert (index , (key , value)) } fn lm_remove (& mut self , index : usize) -> (K , V) { self . remove (index) } fn lm_clear (& mut self) { self . clear () ; } }
};
}
