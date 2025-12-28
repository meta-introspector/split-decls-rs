macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        Error!();
        V3!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        # [cfg (feature = "v3")] impl TryFrom < & str > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k3" , "public" , V3 :: PUBLIC_KEY) ? , phantom : PhantomData , }) } }
    };
}

impl_62!();