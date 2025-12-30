// Generated macro for impl_105 (impl)
macro_rules! Depcrate_adaptors_multi_productimpl_105 {
() => {
// Module: crate::adaptors::multi_product
// Provides: {"impl_105"}
// Dependencies: {}
impl < I > MultiProductIter < I > where I : Iterator + Clone , I :: Item : Clone , { fn new (iter : I) -> Self { Self { iter : iter . clone () , iter_orig : iter , } } }
};
}
