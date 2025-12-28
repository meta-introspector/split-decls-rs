macro_rules! deps {
    () => {
        KeyPair!();
        Error!();
        Seed!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl KeyPair { # [doc = " Import a key pair from an OpenSSL-compatible DER file."] pub fn from_der (der : & [u8]) -> Result < Self , Error > { if der . len () != DER_HEADER_SK . len () + Seed :: BYTES || der [0 .. 16] != DER_HEADER_SK { return Err (Error :: ParseError) ; } let mut seed = [0u8 ; Seed :: BYTES] ; seed . copy_from_slice (& der [16 ..]) ; let kp = KeyPair :: from_seed (Seed :: new (seed)) ; Ok (kp) } # [doc = " Import a key pair from an OpenSSL-compatible PEM file."] pub fn from_pem (pem : & str) -> Result < Self , Error > { let mut it = pem . split ("-----BEGIN PRIVATE KEY-----") ; let _ = it . next () . ok_or (Error :: ParseError) ? ; let inner = it . next () . ok_or (Error :: ParseError) ? ; let mut it = inner . split ("-----END PRIVATE KEY-----") ; let b64 = it . next () . ok_or (Error :: ParseError) ? ; let _ = it . next () . ok_or (Error :: ParseError) ? ; let mut der = [0u8 ; 16 + Seed :: BYTES] ; Base64 :: decode (& mut der , b64 , Some (b"\r\n\t ")) . map_err (| _ | Error :: ParseError) ? ; Self :: from_der (& der) } # [doc = " Export a key pair as an OpenSSL-compatible PEM file."] # [cfg (feature = "std")] pub fn to_pem (& self) -> String { format ! ("{}\n{}\n" , self . sk . to_pem () . trim () , self . pk . to_pem () . trim ()) } }
    };
}

impl_155!();