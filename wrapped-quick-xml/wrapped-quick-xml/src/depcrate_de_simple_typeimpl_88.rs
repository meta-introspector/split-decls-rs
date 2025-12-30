// Generated macro for impl_88 (impl)
macro_rules! Depcrate_de_simple_typeimpl_88 {
() => {
// Module: crate::de::simple_type
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'de , 'a > EnumAccess < 'de > for AtomicDeserializer < 'de , 'a > { type Error = DeError ; type Variant = UnitOnly ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , DeError > where V : DeserializeSeed < 'de > , { let name = seed . deserialize (self) ? ; Ok ((name , UnitOnly)) } }
};
}
