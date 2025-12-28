macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! validated_attr_value_byte {
    () => {
        deps!();
        fn validated_attr_value_byte (byte : u8) -> Result < u8 , Error > { if is_valid_attr_value (byte) { Ok (byte) } else { Err (Error :: InvalidAttributeValue { character : byte as char , }) } }
    };
}

validated_attr_value_byte!();