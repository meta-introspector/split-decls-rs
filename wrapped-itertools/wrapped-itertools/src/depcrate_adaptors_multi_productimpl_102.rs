// Generated macro for impl_102 (impl)
macro_rules! Depcrate_adaptors_multi_productimpl_102 {
() => {
// Module: crate::adaptors::multi_product
// Provides: {"impl_102"}
// Dependencies: {}
impl < I > std :: fmt :: Debug for MultiProductInner < I > where I : Iterator + Clone + std :: fmt :: Debug , I :: Item : Clone + std :: fmt :: Debug , { debug_fmt_fields ! (MultiProductInner , iters , cur) ; }
};
}
