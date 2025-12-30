// Generated macro for impl_188 (impl)
macro_rules! Depcrate_features_serde_de_ownedimpl_188 {
() => {
// Module: crate::features::serde::de_owned
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'de , DE : Decoder > VariantAccess < 'de > for SerdeDecoder < '_ , DE > { type Error = DecodeError ; fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { DeserializeSeed :: deserialize (seed , self) } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { Deserializer :: deserialize_tuple (self , len , visitor) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { Deserializer :: deserialize_tuple (self , fields . len () , visitor) } }
};
}
