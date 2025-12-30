// Generated macro for impl_171 (impl)
macro_rules! Depcrate_features_serde_de_borrowedimpl_171 {
() => {
// Module: crate::features::serde::de_borrowed
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'de , DE : BorrowDecoder < 'de > > VariantAccess < 'de > for SerdeDecoder < '_ , 'de , DE > { type Error = DecodeError ; fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { DeserializeSeed :: deserialize (seed , self) } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { Deserializer :: deserialize_tuple (self , len , visitor) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { Deserializer :: deserialize_tuple (self , fields . len () , visitor) } }
};
}
