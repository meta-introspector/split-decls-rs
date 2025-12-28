macro_rules! deps {
    () => {
        Value!();
        Null!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl From < Null > for Value { # [inline] fn from (_ : Null) -> Self { Self :: Null } }
    };
}

impl_480!();