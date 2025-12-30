// Generated macro for impl_87 (impl)
macro_rules! Depcrate_providerimpl_87 {
() => {
// Module: crate::provider
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , V > From < PluralElements < T > > for PluralElementsPackedCow < 'static , V > where V : VarULE + ? Sized , T : PartialEq + fmt :: Debug , for < 'a > & 'a T : EncodeAsVarULE < V > , { fn from (value : PluralElements < T >) -> Self { let elements = zerovec :: ule :: encode_varule_to_box (& value . map (| s | (FourBitMetadata :: zero () , s))) ; Self { elements : Cow :: Owned (elements) , } } }
};
}
