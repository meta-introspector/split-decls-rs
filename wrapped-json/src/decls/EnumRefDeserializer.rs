macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! EnumRefDeserializer {
    () => {
        deps!();
        struct EnumRefDeserializer < 'de > { variant : & 'de str , value : Option < & 'de Value > , }
    };
}

EnumRefDeserializer!()