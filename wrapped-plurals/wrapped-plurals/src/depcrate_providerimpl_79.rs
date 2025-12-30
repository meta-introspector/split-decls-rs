// Generated macro for impl_79 (impl)
macro_rules! Depcrate_providerimpl_79 {
() => {
// Module: crate::provider
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T > PluralElementsInner < (FourBitMetadata , T) > { fn into_packed < V > (self) -> Box < PluralElementsPackedULE < V > > where T : PartialEq + fmt :: Debug , for < 'a > & 'a T : EncodeAsVarULE < V > , V : VarULE + ? Sized , { zerovec :: ule :: encode_varule_to_box (& PluralElements (self)) } }
};
}
