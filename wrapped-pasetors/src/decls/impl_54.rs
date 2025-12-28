macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
        V2!();
        Error!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl TryFrom < & str > for AsymmetricSecretKey < V2 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let mut buf = validate_paserk_string (value , "k2" , "secret" , V2 :: SECRET_KEY) ? ; let ret = Self :: from (& buf) ? ; buf . iter_mut () . zeroize () ; Ok (ret) } }
    };
}

impl_54!()