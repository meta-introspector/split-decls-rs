// Generated macro for impl_96 (impl)
macro_rules! Depcrate_de_simple_typeimpl_96 {
() => {
// Module: crate::de::simple_type
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'de , 'a > EnumAccess < 'de > for SimpleTypeDeserializer < 'de , 'a > { type Error = DeError ; type Variant = UnitOnly ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , DeError > where V : DeserializeSeed < 'de > , { let name = seed . deserialize (self) ? ; Ok ((name , UnitOnly)) } }
};
}
