// Generated macro for impl_80 (impl)
macro_rules! Depcrate_providerimpl_80 {
() => {
// Module: crate::provider
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , 'data , V > serde :: Deserialize < 'de > for & 'data PluralElementsPackedULE < V > where 'de : 'data , V : VarULE + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { Err (serde :: de :: Error :: custom ("&PluralElementsPackedULE cannot be deserialized from human-readable formats" ,)) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; PluralElementsPackedULE :: < V > :: parse_bytes (bytes) . map_err (serde :: de :: Error :: custom) } } }
};
}
