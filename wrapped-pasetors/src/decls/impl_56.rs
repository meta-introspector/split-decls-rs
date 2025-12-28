macro_rules! deps {
    () => {
        Error!();
        AsymmetricSecretKey!();
        V3!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "v3")] impl TryFrom < & str > for AsymmetricSecretKey < V3 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let buf = validate_paserk_string (value , "k3" , "secret" , V3 :: SECRET_KEY) ? ; let ret = Self { bytes : buf , phantom : PhantomData , } ; Ok (ret) } }
    };
}

impl_56!()