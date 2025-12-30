// Generated macro for impl_125 (impl)
macro_rules! Depcrate_ext_deimpl_125 {
() => {
// Module: crate::ext::de
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'de > de :: EnumAccess < 'de > for EnumRefDeserializer < 'de > { type Error = Error ; type Variant = VariantRefDeserializer < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : de :: DeserializeSeed < 'de > { let variant = self . id . into_deserializer () ; let visitor = VariantRefDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
};
}
