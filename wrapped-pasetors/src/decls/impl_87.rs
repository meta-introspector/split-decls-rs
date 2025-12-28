macro_rules! deps {
    () => {
        V2!();
        LocalToken!();
        Error!();
        SymmetricKey!();
        UntrustedToken!();
        Local!();
        TrustedToken!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl LocalToken { # [doc = " The header and purpose for the local token: `v2.local.`."] pub const HEADER : & 'static str = "v2.local." ; # [doc = " Encrypt and authenticate a message using nonce_key_bytes to derive a nonce"] # [doc = " using BLAKE2b."] pub (crate) fn encrypt_with_derived_nonce (secret_key : & SymmetricKey < V2 > , nonce_key_bytes : & [u8] , message : & [u8] , footer : Option < & [u8] > ,) -> Result < String , Error > { debug_assert ! (nonce_key_bytes . len () == XCHACHA_NONCESIZE) ; let nonce_key = blake2b :: SecretKey :: from_slice (nonce_key_bytes) . unwrap () ; let mut blake2b = blake2b :: Blake2b :: new (& nonce_key , XCHACHA_NONCESIZE) . unwrap () ; blake2b . update (message . as_ref ()) . unwrap () ; let nonce = Nonce :: from_slice (blake2b . finalize () . unwrap () . unprotected_as_bytes ()) . unwrap () ; let f = footer . unwrap_or (& []) ; let pre_auth = pae :: pae (& [Self :: HEADER . as_bytes () , nonce . as_ref () , f]) ? ; let mut out = vec ! [0u8 ; message . len () + POLY1305_OUTSIZE + nonce . len ()] ; let sk = match SecretKey :: from_slice (secret_key . as_bytes ()) { Ok (val) => val , Err (orion :: errors :: UnknownCryptoError) => return Err (Error :: Key) , } ; match seal (& sk , & nonce , message , Some (& pre_auth) , & mut out [nonce . len () ..] ,) { Ok (()) => () , Err (orion :: errors :: UnknownCryptoError) => return Err (Error :: Encryption) , } out [.. nonce . len ()] . copy_from_slice (nonce . as_ref ()) ; let token_no_footer = format ! ("{}{}" , Self :: HEADER , encode_b64 (out) ?) ; if f . is_empty () { Ok (token_no_footer) } else { Ok (format ! ("{}.{}" , token_no_footer , encode_b64 (f) ?)) } } # [doc = " Create a local token."] pub fn encrypt (secret_key : & SymmetricKey < V2 > , message : & [u8] , footer : Option < & [u8] > ,) -> Result < String , Error > { if message . is_empty () { return Err (Error :: EmptyPayload) ; } let mut rng_bytes = [0u8 ; XCHACHA_NONCESIZE] ; getrandom :: fill (& mut rng_bytes) ? ; Self :: encrypt_with_derived_nonce (secret_key , & rng_bytes , message , footer) } # [doc = " Verify and decrypt a local token."] # [doc = ""] # [doc = " If `footer.is_none()`, then it will be validated but not compared to a known value."] # [doc = " If `footer.is_some()`, then it will be validated AND compared to the known value."] pub fn decrypt (secret_key : & SymmetricKey < V2 > , token : & UntrustedToken < Local , V2 > , footer : Option < & [u8] > ,) -> Result < TrustedToken , Error > { validate_footer_untrusted_token (token , footer) ? ; let f = token . untrusted_footer () ; let nc = token . untrusted_message () ; let n = nc [.. XCHACHA_NONCESIZE] . as_ref () ; let c = nc [n . len () ..] . as_ref () ; let pre_auth = pae :: pae (& [Self :: HEADER . as_bytes () , n , f]) ? ; let mut out = vec ! [0u8 ; c . len () - POLY1305_OUTSIZE] ; let sk = match SecretKey :: from_slice (secret_key . as_bytes ()) { Ok (val) => val , Err (orion :: errors :: UnknownCryptoError) => return Err (Error :: Key) , } ; match open (& sk , & Nonce :: from_slice (n) . unwrap () , c , Some (pre_auth . as_ref ()) , & mut out ,) { Ok (()) => TrustedToken :: _new (Self :: HEADER , & out , f , & []) , Err (orion :: errors :: UnknownCryptoError) => Err (Error :: TokenValidation) , } } }
    };
}

impl_87!()