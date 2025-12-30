// Generated macro for impl_284 (impl)
macro_rules! Depcrate_vecimpl_284 {
() => {
// Module: crate::vec
// Provides: {"impl_284"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a , 'de , T > serde :: de :: DeserializeSeed < 'de > for InPlaceSeed < 'a , T > where T : serde :: de :: Deserialize < 'de > , { type Value = () ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { T :: deserialize_in_place (deserializer , self . 0) } }
};
}
