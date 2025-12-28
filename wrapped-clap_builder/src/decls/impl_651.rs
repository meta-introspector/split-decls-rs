macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl From < & '_ & 'static str > for Id { fn from (name : & '_ & 'static str) -> Self { Self (name . into ()) } }
    };
}

impl_651!();