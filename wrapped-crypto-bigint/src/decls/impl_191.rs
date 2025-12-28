macro_rules! deps {
    () => {
        Zero!();
        ByteArray!();
        ArrayEncoding!();
        NonZero!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (feature = "hybrid-array")] impl < T > NonZero < T > where T : ArrayEncoding + Zero , { # [doc = " Decode a non-zero integer from big endian bytes."] pub fn from_be_byte_array (bytes : ByteArray < T >) -> CtOption < Self > { Self :: new (T :: from_be_byte_array (bytes)) } # [doc = " Decode a non-zero integer from big endian bytes."] pub fn from_le_byte_array (bytes : ByteArray < T >) -> CtOption < Self > { Self :: new (T :: from_be_byte_array (bytes)) } }
    };
}

impl_191!()