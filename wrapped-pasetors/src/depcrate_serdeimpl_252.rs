// Generated macro for impl_252 (impl)
macro_rules! Depcrate_serdeimpl_252 {
() => {
// Module: crate::serde
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (all (feature = "paserk" , feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde" , feature = "std"))))] impl < 'de > serde :: Deserialize < 'de > for Id { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_id = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_id) . map_err (serde :: de :: Error :: custom) } }
};
}
