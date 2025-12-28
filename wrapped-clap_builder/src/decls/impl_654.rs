macro_rules! deps {
    () => {
        Str!();
        Id!();
    };
}

macro_rules! impl_654 {
    () => {
        deps!();
        impl From < Id > for String { fn from (name : Id) -> Self { Str :: from (name) . into () } }
    };
}

impl_654!();