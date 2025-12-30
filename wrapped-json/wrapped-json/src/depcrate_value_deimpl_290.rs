// Generated macro for impl_290 (impl)
macro_rules! Depcrate_value_deimpl_290 {
() => {
// Module: crate::value::de
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'de > EnumAccess < 'de > for EnumDeserializer { type Error = Error ; type Variant = VariantDeserializer ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , VariantDeserializer) , Error > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
};
}
