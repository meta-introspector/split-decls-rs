macro_rules! deps {
    () => {
        V4!();
        AsymmetricSecretKey!();
        Error!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl TryFrom < & str > for AsymmetricSecretKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { let mut buf = validate_paserk_string (value , "k4" , "secret" , V4 :: SECRET_KEY) ? ; let ret = Self :: from (& buf) ? ; buf . iter_mut () . zeroize () ; Ok (ret) } }
    };
}

impl_58!()