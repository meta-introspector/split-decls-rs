// Generated macro for PluralElementsUnpacked (struct)
macro_rules! Depcrate_providerPluralElementsUnpacked {
() => {
// Module: crate::provider
// Provides: {"PluralElementsUnpacked"}
// Dependencies: {}
# [doc = " Internal unpacked and deserialized values from a [`PluralElementsPackedULE`]."] # [derive (Debug)] struct PluralElementsUnpacked < 'a , V : VarULE + ? Sized > { pub default : PluralElementWithMetadata < 'a , V > , pub specials : Option < & 'a PluralElementsTupleSliceVarULE < V > > , }
};
}
