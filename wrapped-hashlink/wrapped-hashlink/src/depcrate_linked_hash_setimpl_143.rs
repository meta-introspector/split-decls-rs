// Generated macro for impl_143 (impl)
macro_rules! Depcrate_linked_hash_setimpl_143 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_143"}
// Dependencies: {}
impl < T , S > LinkedHashSet < T , S > { # [inline] pub fn capacity (& self) -> usize { self . map . capacity () } # [inline] pub fn iter (& self) -> Iter < '_ , T > { Iter { iter : self . map . keys () , } } # [inline] pub fn len (& self) -> usize { self . map . len () } # [inline] pub fn is_empty (& self) -> bool { self . map . is_empty () } # [inline] pub fn drain (& mut self) -> Drain < '_ , T > { Drain { iter : self . map . drain () , } } # [inline] pub fn clear (& mut self) { self . map . clear () } # [inline] pub fn retain < F > (& mut self , mut f : F) where F : FnMut (& T) -> bool , { self . map . retain (| k , _ | f (k)) ; } }
};
}
