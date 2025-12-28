macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        Error!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < V : Version > AsymmetricPublicKey < V > { # [doc = " Create a `AsymmetricPublicKey` from `bytes`."] pub fn from (bytes : & [u8]) -> Result < Self , Error > { V :: validate_public_key (bytes) ? ; Ok (Self { bytes : bytes . to_vec () , phantom : PhantomData , }) } # [doc = " Return this as a byte-slice."] pub fn as_bytes (& self) -> & [u8] { self . bytes . as_slice () } }
    };
}

impl_43!();