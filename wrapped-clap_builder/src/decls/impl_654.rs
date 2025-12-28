macro_rules! deps {
    () => {
        Id!();
        Str!();
    };
}

macro_rules! impl_654 {
    () => {
        deps!();
        impl From < Id > for String { fn from (name : Id) -> Self { Str :: from (name) . into () } }
    };
}

impl_654!()