macro_rules! deps {
    () => {
        V4!();
        Error!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Version for V4 { const LOCAL_KEY : usize = 32 ; const SECRET_KEY : usize = 32 + Self :: PUBLIC_KEY ; const PUBLIC_KEY : usize = 32 ; const PUBLIC_SIG : usize = 64 ; const LOCAL_NONCE : usize = 32 ; const LOCAL_TAG : usize = 32 ; const PUBLIC_HEADER : & 'static str = "v4.public." ; const LOCAL_HEADER : & 'static str = "v4.local." ; # [cfg (feature = "paserk")] const PASERK_ID : usize = 44 ; fn validate_local_key (key_bytes : & [u8]) -> Result < () , Error > { if key_bytes . len () != Self :: LOCAL_KEY { return Err (Error :: Key) ; } Ok (()) } fn validate_secret_key (key_bytes : & [u8]) -> Result < () , Error > { if key_bytes . len () != Self :: SECRET_KEY { return Err (Error :: Key) ; } let seed = Seed :: from_slice (& key_bytes [.. 32]) . map_err (| _ | Error :: Key) ? ; let kp = KeyPair :: from_seed (seed) ; if ! bool :: from (kp . pk . as_slice () . ct_eq (& key_bytes [32 ..])) { return Err (Error :: Key) ; } Ok (()) } fn validate_public_key (key_bytes : & [u8]) -> Result < () , Error > { if key_bytes . len () != Self :: PUBLIC_KEY { return Err (Error :: Key) ; } Ok (()) } }
    };
}

impl_109!();