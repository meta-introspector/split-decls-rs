macro_rules! deps {
    () => {
        EncodeUtf8Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl EncodeUtf8Error { # [inline] pub fn description (& self) -> & str { "an error occurred while encoding a utf8 char into the buffer" } }
    };
}

impl_14!()