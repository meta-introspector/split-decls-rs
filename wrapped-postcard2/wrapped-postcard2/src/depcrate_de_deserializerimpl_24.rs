// Generated macro for impl_24 (impl)
macro_rules! Depcrate_de_deserializerimpl_24 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , 'b : 'a , F : Flavor < 'b > > serde :: de :: MapAccess < 'b > for MapAccess < 'a , 'b , F > { type Error = Error ; # [inline] fn next_key_seed < K : DeserializeSeed < 'b > > (& mut self , seed : K) -> Result < Option < K :: Value > > { if self . len > 0 { self . len -= 1 ; Ok (Some (DeserializeSeed :: deserialize (seed , & mut * self . deserializer ,) ?)) } else { Ok (None) } } # [inline] fn next_value_seed < V : DeserializeSeed < 'b > > (& mut self , seed : V) -> Result < V :: Value > { DeserializeSeed :: deserialize (seed , & mut * self . deserializer) } # [inline] fn size_hint (& self) -> Option < usize > { Some (self . len) } }
};
}
