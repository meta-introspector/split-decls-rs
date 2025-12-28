macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < const N : usize > From < [u8 ; N] > for Data { fn from (from : [u8 ; N]) -> Self { Self :: from_slice (& from) } }
    };
}

impl_92!()