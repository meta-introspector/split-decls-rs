macro_rules! deps {
    () => {
        StringComponent!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 's > StringComponent < 's > { # [inline] fn serialized_size (& self) -> usize { match * self { StringComponent :: Value (s) => s . len () , StringComponent :: Ref (_) => STRING_REF_ENCODED_SIZE , } } # [inline] fn serialize < 'b > (& self , bytes : & 'b mut [u8]) -> & 'b mut [u8] { match * self { StringComponent :: Value (s) => { bytes [.. s . len ()] . copy_from_slice (s . as_bytes ()) ; & mut bytes [s . len () ..] } StringComponent :: Ref (string_id) => { assert ! (STRING_REF_ENCODED_SIZE == 9) ; bytes [0] = STRING_REF_TAG ; bytes [1 .. 9] . copy_from_slice (& string_id . 0 . to_le_bytes ()) ; & mut bytes [9 ..] } } } }
    };
}

impl_87!();