// Generated macro for impl_170 (impl)
macro_rules! Depcrate_features_serde_de_borrowedimpl_170 {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'de , DE : BorrowDecoder < 'de > > EnumAccess < 'de > for SerdeDecoder < '_ , 'de , DE > { type Error = DecodeError ; type Variant = Self ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let idx = u32 :: decode (& mut self . de) ? ; let val = seed . deserialize (idx . into_deserializer ()) ? ; Ok ((val , self)) } }
};
}
