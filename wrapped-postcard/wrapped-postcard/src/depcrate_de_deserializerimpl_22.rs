// Generated macro for impl_22 (impl)
macro_rules! Depcrate_de_deserializerimpl_22 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , 'b : 'a , F : Flavor < 'b > > serde :: de :: SeqAccess < 'b > for SeqAccess < 'a , 'b , F > { type Error = Error ; # [inline] fn next_element_seed < V : DeserializeSeed < 'b > > (& mut self , seed : V) -> Result < Option < V :: Value > > { if self . len > 0 { self . len -= 1 ; Ok (Some (DeserializeSeed :: deserialize (seed , & mut * self . deserializer ,) ?)) } else { Ok (None) } } # [inline] fn size_hint (& self) -> Option < usize > { match self . deserializer . flavor . size_hint () { Some (size) if size < self . len => None , _ => Some (self . len) , } } }
};
}
