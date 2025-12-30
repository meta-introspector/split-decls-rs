// Generated macro for impl_306 (impl)
macro_rules! Depcrate_value_deimpl_306 {
() => {
// Module: crate::value::de
// Provides: {"impl_306"}
// Dependencies: {}
impl < 'de > EnumAccess < 'de > for EnumRefDeserializer < 'de > { type Error = Error ; type Variant = VariantRefDeserializer < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Error > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantRefDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
};
}
