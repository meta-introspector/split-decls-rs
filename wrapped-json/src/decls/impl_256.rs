macro_rules! deps {
    () => {
        Error!();
        EnumDeserializer!();
        Result!();
        VariantDeserializer!();
        Value!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'de > EnumAccess < 'de > for EnumDeserializer { type Error = Error ; type Variant = VariantDeserializer ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , VariantDeserializer) , Error > where V : DeserializeSeed < 'de > , { let variant = self . variant . into_deserializer () ; let visitor = VariantDeserializer { value : self . value } ; seed . deserialize (variant) . map (| v | (v , visitor)) } }
    };
}

impl_256!();