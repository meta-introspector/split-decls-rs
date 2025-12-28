macro_rules! deps {
    () => {
        Error!();
        V2!();
        AsymmetricPublicKey!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl TryFrom < & str > for AsymmetricPublicKey < V2 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k2" , "public" , V2 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
    };
}

impl_60!();