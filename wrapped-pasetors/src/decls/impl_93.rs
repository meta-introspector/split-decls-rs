macro_rules! deps {
    () => {
        Error!();
        V3!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Version for V3 { const LOCAL_KEY : usize = 32 ; const SECRET_KEY : usize = 48 ; const PUBLIC_KEY : usize = 49 ; const PUBLIC_SIG : usize = 96 ; const LOCAL_NONCE : usize = 32 ; const LOCAL_TAG : usize = 48 ; const PUBLIC_HEADER : & 'static str = "v3.public." ; const LOCAL_HEADER : & 'static str = "v3.local." ; # [cfg (feature = "paserk")] const PASERK_ID : usize = 44 ; fn validate_local_key (_key_bytes : & [u8]) -> Result < () , Error > { unimplemented ! () ; } fn validate_secret_key (key_bytes : & [u8]) -> Result < () , Error > { if key_bytes . len () != Self :: SECRET_KEY { return Err (Error :: Key) ; } Ok (()) } fn validate_public_key (key_bytes : & [u8]) -> Result < () , Error > { if key_bytes . len () != Self :: PUBLIC_KEY { return Err (Error :: Key) ; } if key_bytes [0] != 0x02 && key_bytes [0] != 0x03 { return Err (Error :: Key) ; } Ok (()) } }
    };
}

impl_93!()