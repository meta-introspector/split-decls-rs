macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for & 'a PotentialUtf8 { # [inline] fn from (other : & 'a str) -> Self { PotentialUtf8 :: from_str (other) } }
    };
}

impl_24!()