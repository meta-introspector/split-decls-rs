macro_rules! deps {
    () => {
        Error!();
        PublicKey!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl PublicKey { # [doc = " Import a public key from an OpenSSL-compatible DER file."] pub fn from_der (der : & [u8]) -> Result < Self , Error > { if der . len () != DER_HEADER_PK . len () + PublicKey :: BYTES || der [0 .. 12] != DER_HEADER_PK { return Err (Error :: ParseError) ; } let mut pk = [0u8 ; PublicKey :: BYTES] ; pk . copy_from_slice (& der [12 ..]) ; let pk = PublicKey :: new (pk) ; Ok (pk) } # [doc = " Import a public key from an OpenSSL-compatible PEM file."] pub fn from_pem (pem : & str) -> Result < Self , Error > { let mut it = pem . split ("-----BEGIN PUBLIC KEY-----") ; let _ = it . next () . ok_or (Error :: ParseError) ? ; let inner = it . next () . ok_or (Error :: ParseError) ? ; let mut it = inner . split ("-----END PUBLIC KEY-----") ; let b64 = it . next () . ok_or (Error :: ParseError) ? ; let _ = it . next () . ok_or (Error :: ParseError) ? ; let mut der = [0u8 ; 12 + PublicKey :: BYTES] ; Base64 :: decode (& mut der , b64 , Some (b"\r\n\t ")) . map_err (| _ | Error :: ParseError) ? ; Self :: from_der (& der) } # [doc = " Export a public key as an OpenSSL-compatible DER file."] # [cfg (feature = "std")] pub fn to_der (& self) -> Vec < u8 > { let mut der = [0u8 ; 12 + PublicKey :: BYTES] ; der [0 .. 12] . copy_from_slice (& DER_HEADER_PK) ; der [12 ..] . copy_from_slice (self . as_ref ()) ; der . to_vec () } # [doc = " Export a public key as an OpenSSL-compatible PEM file."] # [cfg (feature = "std")] pub fn to_pem (& self) -> String { let b64 = Base64 :: encode_to_string (self . to_der ()) . unwrap () ; format ! ("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n" , b64) } }
    };
}

impl_157!();