// Generated macro for impl_108 (impl)
macro_rules! Depcrate_de_textimpl_108 {
() => {
// Module: crate::de::text
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'de > VariantAccess < 'de > for TextDeserializer < 'de > { type Error = DeError ; # [inline] fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { seed . deserialize (self) } # [inline] fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { self . deserialize_tuple (len , visitor) } # [inline] fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { self . deserialize_struct ("" , fields , visitor) } }
};
}
