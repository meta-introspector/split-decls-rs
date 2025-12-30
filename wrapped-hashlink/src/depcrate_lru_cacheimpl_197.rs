// Generated macro for impl_197 (impl)
macro_rules! Depcrate_lru_cacheimpl_197 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_197"}
// Dependencies: {}
impl < K , V , S > LruCache < K , V , S > { # [inline] pub fn with_hasher (capacity : usize , hash_builder : S) -> Self { LruCache { map : LinkedHashMap :: with_hasher (hash_builder) , max_size : capacity , } } # [inline] pub fn capacity (& self) -> usize { self . max_size } # [inline] pub fn len (& self) -> usize { self . map . len () } # [inline] pub fn is_empty (& self) -> bool { self . map . is_empty () } # [inline] pub fn clear (& mut self) { self . map . clear () ; } # [inline] pub fn iter (& self) -> Iter < '_ , K , V > { self . map . iter () } # [inline] pub fn iter_mut (& mut self) -> IterMut < '_ , K , V > { self . map . iter_mut () } # [inline] pub fn drain (& mut self) -> Drain < '_ , K , V > { self . map . drain () } # [inline] pub fn retain < F > (& mut self , f : F) where F : FnMut (& K , & mut V) -> bool , { self . map . retain (f) ; } }
};
}
