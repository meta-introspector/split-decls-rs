// Generated macro for impl_130 (impl)
macro_rules! Depcrate_mapimpl_130 {
() => {
// Module: crate::map
// Provides: {"impl_130"}
// Dependencies: {}
impl Hash for Map < String , Value > { fn hash < H : Hasher > (& self , state : & mut H) { # [cfg (not (feature = "preserve_order"))] { self . map . hash (state) ; } # [cfg (feature = "preserve_order")] { let mut kv = Vec :: from_iter (& self . map) ; kv . sort_unstable_by (| a , b | a . 0 . cmp (b . 0)) ; kv . hash (state) ; } } }
};
}
