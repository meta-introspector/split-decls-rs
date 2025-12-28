macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl From < i64 > for Value { # [inline] fn from (i : i64) -> Self { Self :: Integer (i) } }
    };
}

impl_492!()