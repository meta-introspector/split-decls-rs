// Generated macro for impl_26 (impl)
macro_rules! Depcrate_de_deserializerimpl_26 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'de , F : Flavor < 'de > > serde :: de :: VariantAccess < 'de > for & mut Deserializer < 'de , F > { type Error = Error ; # [inline] fn unit_variant (self) -> Result < () > { Ok (()) } # [inline] fn newtype_variant_seed < V : DeserializeSeed < 'de > > (self , seed : V) -> Result < V :: Value > { DeserializeSeed :: deserialize (seed , self) } # [inline] fn tuple_variant < V : Visitor < 'de > > (self , len : usize , visitor : V) -> Result < V :: Value > { serde :: de :: Deserializer :: deserialize_tuple (self , len , visitor) } # [inline] fn struct_variant < V : Visitor < 'de > > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value > { serde :: de :: Deserializer :: deserialize_tuple (self , fields . len () , visitor) } }
};
}
