// Generated macro for impl_90 (impl)
macro_rules! Depcrate_store_slice_implimpl_90 {
() => {
// Module: crate::store::slice_impl
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Store < K , V > for & 'a [(K , V)] { # [inline] fn lm_len (& self) -> usize { self . len () } # [inline] fn lm_is_empty (& self) -> bool { self . is_empty () } # [inline] fn lm_get (& self , index : usize) -> Option < (& K , & V) > { self . get (index) . map (map_f) } # [inline] fn lm_last (& self) -> Option < (& K , & V) > { self . last () . map (map_f) } # [inline] fn lm_binary_search_by < F > (& self , mut cmp : F) -> Result < usize , usize > where F : FnMut (& K) -> Ordering , { self . binary_search_by (| (k , _) | cmp (k)) } }
};
}
