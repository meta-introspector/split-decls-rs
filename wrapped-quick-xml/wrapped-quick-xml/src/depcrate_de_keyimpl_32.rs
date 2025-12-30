// Generated macro for impl_32 (impl)
macro_rules! Depcrate_de_keyimpl_32 {
() => {
// Module: crate::de::key
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de , 'd > EnumAccess < 'de > for QNameDeserializer < 'de , 'd > { type Error = DeError ; type Variant = UnitOnly ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let name = seed . deserialize (self) ? ; Ok ((name , UnitOnly)) } }
};
}
