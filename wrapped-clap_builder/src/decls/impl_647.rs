macro_rules! deps {
    () => {
        Id!();
        Str!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl From < & '_ Str > for Id { fn from (name : & '_ Str) -> Self { Self (name . into ()) } }
    };
}

impl_647!();