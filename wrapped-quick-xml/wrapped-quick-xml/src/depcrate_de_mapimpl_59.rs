// Generated macro for impl_59 (impl)
macro_rules! Depcrate_de_mapimpl_59 {
() => {
// Module: crate::de::map
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'de , 'd , R , E > de :: EnumAccess < 'de > for ElementDeserializer < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let name = seed . deserialize (QNameDeserializer :: from_elem (& self . start) ?) ? ; Ok ((name , self)) } }
};
}
