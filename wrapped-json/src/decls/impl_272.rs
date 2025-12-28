macro_rules! deps {
    () => {
        Error!();
        Result!();
        VariantRefDeserializer!();
        EnumRefDeserializer!();
        Value!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < 'de > EnumAccess < 'de > for EnumRefDeserializer < 'de > { type Error = Error ; type Variant = VariantRefDeserializer < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Error > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantRefDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
    };
}

impl_272!();