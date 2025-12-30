// Generated macro for impl_27 (impl)
macro_rules! Depcrate_de_deserializerimpl_27 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'de , F : Flavor < 'de > > serde :: de :: EnumAccess < 'de > for & mut Deserializer < 'de , F > { type Error = Error ; type Variant = Self ; # [inline] fn variant_seed < V : DeserializeSeed < 'de > > (self , seed : V) -> Result < (V :: Value , Self) > { let varint = self . try_take_varint_u32 () ? ; let v = DeserializeSeed :: deserialize (seed , varint . into_deserializer ()) ? ; Ok ((v , self)) } }
};
}
