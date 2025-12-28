macro_rules! deps {
    () => {
        Value!();
        Data!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl From < & [u8] > for Value { fn from (from : & [u8]) -> Self { Self { data : Data :: from_slice (from) , ty : Type :: Bytes , } } }
    };
}

impl_79!()