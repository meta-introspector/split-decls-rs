macro_rules! deps {
    () => {
        Id!();
        Str!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl From < Str > for Id { fn from (name : Str) -> Self { Self (name) } }
    };
}

impl_646!();