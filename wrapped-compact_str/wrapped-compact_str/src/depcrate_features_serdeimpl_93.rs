// Generated macro for impl_93 (impl)
macro_rules! Depcrate_features_serdeimpl_93 {
() => {
// Module: crate::features::serde
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl < 'de > serde :: Deserialize < 'de > for CompactString { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { compact_string (deserializer) } }
};
}
