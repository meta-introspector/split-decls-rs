macro_rules! deps {
    () => {
        Boolean!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < Boolean > for bool { fn from (b : Boolean) -> Self { b . 0 } }
    };
}

impl_8!()