macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! EnumDeserializer {
    () => {
        deps!();
        struct EnumDeserializer { variant : String , value : Option < Value > , }
    };
}

EnumDeserializer!();