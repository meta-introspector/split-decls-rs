macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl From < isize > for Value { # [inline] fn from (i : isize) -> Self { Self :: Integer (i as i64) } }
    };
}

impl_482!()