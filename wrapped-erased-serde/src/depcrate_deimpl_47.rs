// Generated macro for impl_47 (impl)
macro_rules! Depcrate_deimpl_47 {
() => {
// Module: crate::de
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut (dyn DeserializeSeed < 'de > + '_) { type Value = Out ; fn deserialize < D > (self , deserializer : D) -> Result < Out , D :: Error > where D : serde :: Deserializer < 'de > , { let mut erased = erase :: Deserializer :: new (deserializer) ; self . erased_deserialize_seed (& mut erased) . map_err (unerase) } }
};
}
