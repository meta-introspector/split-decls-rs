macro_rules! deps {
    () => {
        NistP384!();
        FieldBytes!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl FieldBytesEncoding < NistP384 > for U384 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U384 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
    };
}

impl_40!();