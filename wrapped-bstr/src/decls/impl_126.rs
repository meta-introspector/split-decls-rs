macro_rules! deps {
    () => {
        FromUtf8Error!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl error :: Error for FromUtf8Error { # [inline] fn description (& self) -> & str { "invalid UTF-8 vector" } }
    };
}

impl_126!();