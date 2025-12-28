macro_rules! deps {
    () => {
        U32X4!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl U32X4 { # [inline] fn from (bytes : & [u8]) -> Self { U32X4 ([u32 :: from (bytes [0]) , u32 :: from (bytes [1]) , u32 :: from (bytes [2]) , u32 :: from (bytes [3]) ,]) } }
    };
}

impl_3!();