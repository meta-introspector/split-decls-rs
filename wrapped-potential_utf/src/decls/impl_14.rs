macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl From < char > for PotentialCodePoint { # [inline] fn from (value : char) -> Self { Self :: from_char (value) } }
    };
}

impl_14!();