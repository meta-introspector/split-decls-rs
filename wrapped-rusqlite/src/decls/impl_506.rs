macro_rules! deps {
    () => {
        Blob!();
        ValueRef!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for ValueRef < 'a > { # [inline] fn from (s : & [u8]) -> ValueRef < '_ > { ValueRef :: Blob (s) } }
    };
}

impl_506!();