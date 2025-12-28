macro_rules! deps {
    () => {
        Error!();
        V4!();
        AsymmetricPublicKey!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl TryFrom < & str > for AsymmetricPublicKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k4" , "public" , V4 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
    };
}

impl_64!();