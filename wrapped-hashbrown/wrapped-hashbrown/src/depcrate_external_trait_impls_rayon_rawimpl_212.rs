// Generated macro for impl_212 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_212 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_212"}
// Dependencies: {}
impl < T > UnindexedProducer for ParIterProducer < T > { type Item = Bucket < T > ; # [cfg_attr (feature = "inline-more" , inline)] fn split (self) -> (Self , Option < Self >) { let (left , right) = self . iter . split () ; let left = ParIterProducer { iter : left } ; let right = right . map (| right | ParIterProducer { iter : right }) ; (left , right) } # [cfg_attr (feature = "inline-more" , inline)] fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . iter) } }
};
}
