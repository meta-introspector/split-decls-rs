// Generated macro for impl_76 (impl)
macro_rules! Depcrate_providerimpl_76 {
() => {
// Module: crate::provider
// Provides: {"impl_76"}
// Dependencies: {}
impl < T > PluralElementsInner < (FourBitMetadata , T) > where T : PartialEq , { fn to_packed_builder < 'a , V > (& 'a self) -> PluralElementsPackedBuilder < 'a , T > where & 'a T : EncodeAsVarULE < V > , V : VarULE + ? Sized , { let specials = self . get_specials_tuples () . map (| (plural_category , (metadata , t)) | VarTuple { sized : PluralCategoryAndMetadata { plural_category , metadata : * metadata , } , variable : t , }) . collect :: < Vec < _ > > () ; PluralElementsPackedBuilder { default : (self . other . 0 , & self . other . 1) , specials : if specials . is_empty () { None } else { Some (specials) } , } } }
};
}
