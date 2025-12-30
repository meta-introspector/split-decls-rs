// Generated macro for impl_187 (impl)
macro_rules! Depcrate_features_serde_de_ownedimpl_187 {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'de , DE : Decoder > EnumAccess < 'de > for SerdeDecoder < '_ , DE > { type Error = DecodeError ; type Variant = Self ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let idx = u32 :: decode (& mut self . de) ? ; let val = seed . deserialize (idx . into_deserializer ()) ? ; Ok ((val , self)) } }
};
}
