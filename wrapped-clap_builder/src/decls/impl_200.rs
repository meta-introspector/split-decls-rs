macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl From < Str > for String { fn from (name : Str) -> Self { name . name . into_string () } }
    };
}

impl_200!();