// Generated macro for impl_116 (impl)
macro_rules! Depcrate_ext_deimpl_116 {
() => {
// Module: crate::ext::de
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'de , U : ValueBase < 'de > + ValueExt > de :: EnumAccess < 'de > for EnumDeserializer < U > { type Error = Error ; type Variant = VariantDeserializer < U > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : de :: DeserializeSeed < 'de > { let variant = self . id . into_deserializer () ; let visitor = VariantDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
};
}
