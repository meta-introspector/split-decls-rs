macro_rules! deps {
    () => {
        EncodeUtf8Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Error for EncodeUtf8Error { # [inline] fn description (& self) -> & str { EncodeUtf8Error :: description (self) } }
    };
}

impl_13!()