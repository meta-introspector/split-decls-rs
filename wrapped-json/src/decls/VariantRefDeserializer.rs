macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! VariantRefDeserializer {
    () => {
        deps!();
        struct VariantRefDeserializer < 'de > { value : Option < & 'de Value > , }
    };
}

VariantRefDeserializer!()