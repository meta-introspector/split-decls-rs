macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_650 {
    () => {
        deps!();
        impl From < & 'static str > for Id { fn from (name : & 'static str) -> Self { Self (name . into ()) } }
    };
}

impl_650!();