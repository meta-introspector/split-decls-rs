macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
        V2!();
        Error!();
        V4!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < V : Version > AsymmetricSecretKey < V > { # [doc = " Create a `AsymmetricSecretKey` from `bytes`."] # [doc = ""] # [doc = " __PANIC__: If the version is V2 or V4, a panic will occur if an all-zero"] # [doc = " secret seed is used."] pub fn from (bytes : & [u8]) -> Result < Self , Error > { V :: validate_secret_key (bytes) ? ; Ok (Self { bytes : bytes . to_vec () , phantom : PhantomData , }) } # [doc = " Return this as a byte-slice."] pub fn as_bytes (& self) -> & [u8] { self . bytes . as_slice () } }
    };
}

impl_38!()