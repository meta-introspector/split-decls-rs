// Generated macro for impl_367 (impl)
macro_rules! Depcrate_value_deimpl_367 {
() => {
// Module: crate::value::de
// Provides: {"impl_367"}
// Dependencies: {}
impl < 'de > EnumAccess < 'de > for EnumRefDeserializer < 'de > { type Error = Error ; type Variant = VariantRefDeserializer < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Error > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantRefDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
};
}
