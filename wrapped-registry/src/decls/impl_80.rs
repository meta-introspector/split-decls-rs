macro_rules! deps {
    () => {
        Data!();
        Value!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < const N : usize > From < [u8 ; N] > for Value { fn from (from : [u8 ; N]) -> Self { Self { data : Data :: from_slice (& from) , ty : Type :: Bytes , } } }
    };
}

impl_80!()