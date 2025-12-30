// Generated macro for impl_222 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_222 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_222"}
// Dependencies: {}
impl < T : Send > UnindexedProducer for ParDrainProducer < T > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn split (self) -> (Self , Option < Self >) { let (left , right) = self . iter . clone () . split () ; mem :: forget (self) ; let left = ParDrainProducer { iter : left } ; let right = right . map (| right | ParDrainProducer { iter : right }) ; (left , right) } # [cfg_attr (feature = "inline-more" , inline)] fn fold_with < F > (mut self , mut folder : F) -> F where F : Folder < Self :: Item > , { for item in & mut self . iter { folder = folder . consume (unsafe { item . read () }) ; if folder . full () { return folder ; } } mem :: forget (self) ; folder } }
};
}
