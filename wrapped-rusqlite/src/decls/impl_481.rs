macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_481 {
    () => {
        deps!();
        impl From < bool > for Value { # [inline] fn from (i : bool) -> Self { Self :: Integer (i as i64) } }
    };
}

impl_481!()