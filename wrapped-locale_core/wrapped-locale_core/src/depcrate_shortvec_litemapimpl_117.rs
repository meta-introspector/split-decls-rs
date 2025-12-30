// Generated macro for impl_117 (impl)
macro_rules! Depcrate_shortvec_litemapimpl_117 {
() => {
// Module: crate::shortvec::litemap
// Provides: {"impl_117"}
// Dependencies: {}
impl < K , V > Store < K , V > for ShortBoxSlice < (K , V) > { # [inline] fn lm_len (& self) -> usize { self . len () } # [inline] fn lm_is_empty (& self) -> bool { use ShortBoxSliceInner :: * ; matches ! (self . 0 , ZeroOne (None)) } # [inline] fn lm_get (& self , index : usize) -> Option < (& K , & V) > { self . get (index) . map (| elt | (& elt . 0 , & elt . 1)) } # [inline] fn lm_last (& self) -> Option < (& K , & V) > { use ShortBoxSliceInner :: * ; match self . 0 { ZeroOne (ref v) => v . as_ref () , # [cfg (feature = "alloc")] Multi (ref v) => v . last () , # [cfg (not (feature = "alloc"))] Two ([_ , ref v]) => Some (v) , } . map (| elt | (& elt . 0 , & elt . 1)) } # [inline] fn lm_binary_search_by < F > (& self , mut cmp : F) -> Result < usize , usize > where F : FnMut (& K) -> core :: cmp :: Ordering , { self . binary_search_by (| (k , _) | cmp (k)) } }
};
}
