macro_rules! deps {
    () => {
        InitError!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for InitError { fn from (s : & 'a str) -> Self { s . to_owned () . into () } }
    };
}

impl_99!();