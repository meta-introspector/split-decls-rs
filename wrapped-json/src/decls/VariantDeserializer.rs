macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! VariantDeserializer {
    () => {
        deps!();
        struct VariantDeserializer { value : Option < Value > , }
    };
}

VariantDeserializer!();