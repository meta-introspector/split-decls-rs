macro_rules! deps {
    () => {
        UntrustedToken!();
        AsymmetricSecretKey!();
        AsymmetricPublicKey!();
        Error!();
        PublicToken!();
        TrustedToken!();
        V4!();
        Public!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl PublicToken { # [doc = " The header and purpose for the public token: `v4.public.`."] pub const HEADER : & 'static str = "v4.public." ; # [doc = " Create a public token."] pub fn sign (secret_key : & AsymmetricSecretKey < V4 > , message : & [u8] , footer : Option < & [u8] > , implicit_assert : Option < & [u8] > ,) -> Result < String , Error > { if message . is_empty () { return Err (Error :: EmptyPayload) ; } let sk = SecretKey :: from_slice (secret_key . as_bytes ()) . map_err (| _ | Error :: Key) ? ; let f = footer . unwrap_or (& []) ; let i = implicit_assert . unwrap_or (& []) ; let m2 = pae :: pae (& [Self :: HEADER . as_bytes () , message , f , i]) ? ; let sig = sk . sign (m2 , None) ; let mut m_sig : Vec < u8 > = Vec :: from (message) ; m_sig . extend_from_slice (sig . as_ref ()) ; let token_no_footer = format ! ("{}{}" , Self :: HEADER , encode_b64 (m_sig) ?) ; if f . is_empty () { Ok (token_no_footer) } else { Ok (format ! ("{}.{}" , token_no_footer , encode_b64 (f) ?)) } } # [doc = " Verify a public token."] # [doc = ""] # [doc = " If `footer.is_none()`, then it will be validated but not compared to a known value."] # [doc = " If `footer.is_some()`, then it will be validated AND compared to the known value."] pub fn verify (public_key : & AsymmetricPublicKey < V4 > , token : & UntrustedToken < Public , V4 > , footer : Option < & [u8] > , implicit_assert : Option < & [u8] > ,) -> Result < TrustedToken , Error > { validate_footer_untrusted_token (token , footer) ? ; let f = token . untrusted_footer () ; let i = implicit_assert . unwrap_or (& []) ; let sm = token . untrusted_message () ; let m = token . untrusted_payload () ; let s = sm [m . len () .. m . len () + V4 :: PUBLIC_SIG] . as_ref () ; let m2 = pae :: pae (& [Self :: HEADER . as_bytes () , m , f , i]) ? ; let pk : PublicKey = PublicKey :: from_slice (public_key . as_bytes ()) . map_err (| _ | Error :: Key) ? ; debug_assert ! (s . len () == V4 :: PUBLIC_SIG) ; let sig = Signature :: from_slice (s) . map_err (| _ | Error :: TokenValidation) ? ; if pk . verify (m2 , & sig) . is_ok () { TrustedToken :: _new (Self :: HEADER , m , f , i) } else { Err (Error :: TokenValidation) } } }
    };
}

impl_114!()