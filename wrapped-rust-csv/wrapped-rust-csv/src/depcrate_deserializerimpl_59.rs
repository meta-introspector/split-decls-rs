// Generated macro for impl_59 (impl)
macro_rules! Depcrate_deserializerimpl_59 {
() => {
// Module: crate::deserializer
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , 'de : 'a , T : DeRecord < 'de > > EnumAccess < 'de > for & 'a mut DeRecordWrap < T > { type Error = DeserializeError ; type Variant = Self ; fn variant_seed < V : DeserializeSeed < 'de > > (self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > { let variant_name = self . next_field () ? ; seed . deserialize (variant_name . into_deserializer ()) . map (| v | (v , self)) } }
};
}
