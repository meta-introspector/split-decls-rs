macro_rules! deps {
    () => {
        Zero!();
        Encoding!();
        NonZero!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < T > NonZero < T > where T : Encoding + Zero , { # [doc = " Decode from big endian bytes."] pub fn from_be_bytes (bytes : T :: Repr) -> CtOption < Self > { Self :: new (T :: from_be_bytes (bytes)) } # [doc = " Decode from little endian bytes."] pub fn from_le_bytes (bytes : T :: Repr) -> CtOption < Self > { Self :: new (T :: from_le_bytes (bytes)) } }
    };
}

impl_183!()