// Generated macro for impl_92 (impl)
macro_rules! Depcrate_features_serdeimpl_92 {
() => {
// Module: crate::features::serde
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl serde :: Serialize for CompactString { fn serialize < S : serde :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { self . as_str () . serialize (serializer) } }
};
}
