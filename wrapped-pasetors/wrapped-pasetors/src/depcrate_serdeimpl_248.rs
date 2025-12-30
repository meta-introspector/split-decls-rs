// Generated macro for impl_248 (impl)
macro_rules! Depcrate_serdeimpl_248 {
() => {
// Module: crate::serde
// Provides: {"impl_248"}
// Dependencies: {}
# [cfg (all (feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "serde" , feature = "std"))))] impl < 'de , V > serde :: Deserialize < 'de > for AsymmetricSecretKey < V > where AsymmetricSecretKey < V > : TryFrom < & 'de str > , < AsymmetricSecretKey < V > as TryFrom < & 'de str > > :: Error : std :: fmt :: Display , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_string = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_string) . map_err (serde :: de :: Error :: custom) } }
};
}
