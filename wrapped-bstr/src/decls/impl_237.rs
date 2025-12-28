macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Utf8Error { fn description (& self) -> & str { "invalid UTF-8" } }
    };
}

impl_237!();