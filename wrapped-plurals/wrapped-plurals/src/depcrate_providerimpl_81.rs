// Generated macro for impl_81 (impl)
macro_rules! Depcrate_providerimpl_81 {
() => {
// Module: crate::provider
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , V > serde :: Deserialize < 'de > for Box < PluralElementsPackedULE < V > > where V : VarULE + ? Sized , Box < V > : serde :: Deserialize < 'de > + PartialEq + fmt :: Debug , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { let plural_elements : PluralElementsInner < (FourBitMetadata , Box < V >) > = PluralElementsInner :: deserialize (deserializer) ? ; Ok (plural_elements . into_packed ()) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; PluralElementsPackedULE :: < V > :: parse_bytes (bytes) . map (| ule | ule . to_owned ()) . map_err (serde :: de :: Error :: custom) } } }
};
}
