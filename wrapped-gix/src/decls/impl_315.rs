macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl From < Buffer < '_ > > for Vec < u8 > { fn from (mut value : Buffer < '_ >) -> Self { std :: mem :: take (& mut value . inner) } }
    };
}

impl_315!();