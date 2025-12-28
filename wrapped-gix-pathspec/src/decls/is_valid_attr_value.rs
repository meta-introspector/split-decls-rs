macro_rules! is_valid_attr_value {
    () => {
        fn is_valid_attr_value (byte : u8) -> bool { byte . is_ascii_alphanumeric () || b",-_" . contains (& byte) }
    };
}

is_valid_attr_value!()