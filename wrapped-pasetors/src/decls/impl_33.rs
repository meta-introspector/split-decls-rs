macro_rules! deps {
    () => {
        Error!();
        SymmetricKey!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < V : Version > SymmetricKey < V > { # [doc = " Create a `SymmetricKey` from `bytes`."] pub fn from (bytes : & [u8]) -> Result < Self , Error > { V :: validate_local_key (bytes) ? ; Ok (Self { bytes : bytes . to_vec () , phantom : PhantomData , }) } # [doc = " Return this as a byte-slice."] pub fn as_bytes (& self) -> & [u8] { self . bytes . as_slice () } }
    };
}

impl_33!()