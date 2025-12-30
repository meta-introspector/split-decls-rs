// Generated macro for impl_101 (impl)
macro_rules! Depcrate_store_vec_implimpl_101 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_101"}
// Dependencies: {}
impl < K , V > Store < K , V > for Vec < (K , V) > { # [inline] fn lm_len (& self) -> usize { self . as_slice () . len () } # [inline] fn lm_is_empty (& self) -> bool { self . as_slice () . is_empty () } # [inline] fn lm_get (& self , index : usize) -> Option < (& K , & V) > { self . as_slice () . get (index) . map (map_f) } # [inline] fn lm_last (& self) -> Option < (& K , & V) > { self . as_slice () . last () . map (map_f) } # [inline] fn lm_binary_search_by < F > (& self , mut cmp : F) -> Result < usize , usize > where F : FnMut (& K) -> Ordering , { self . as_slice () . binary_search_by (| (k , _) | cmp (k)) } }
};
}
