macro_rules! deps {
    () => {
        UnknownCryptoError!();
        PublicKey!();
        FieldElement!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl PublicKey { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Construct from a given byte slice."] pub fn from_slice (slice : & [u8]) -> Result < Self , UnknownCryptoError > { let slice_len = slice . len () ; if slice_len != PUBLIC_KEY_SIZE { return Err (UnknownCryptoError) ; } Ok (Self { fe : FieldElement :: from_bytes (slice . try_into () . unwrap ()) , }) } # [inline] # [doc = " Return the length of the object."] pub fn len (& self) -> usize { PUBLIC_KEY_SIZE } # [inline] # [doc = " Return `true` if this object does not hold any data, `false` otherwise."] # [doc = ""] # [doc = " __NOTE__: This method should always return `false`, since there shouldn't be a way"] # [doc = " to create an empty instance of this object."] pub fn is_empty (& self) -> bool { PUBLIC_KEY_SIZE == 0 } # [inline] # [doc = " Convert this PublicKey to its byte-representation."] pub fn to_bytes (& self) -> [u8 ; 32] { self . fe . as_bytes () } }
    };
}

impl_384!()