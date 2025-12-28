macro_rules! deps {
    () => {
        StoreBytes!();
        BSwap!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < W : StoreBytes + BSwap + Copy , G > StoreBytes for x2 < W , G > { # [inline (always)] unsafe fn unsafe_read_le (input : & [u8]) -> Self { let input = input . split_at (input . len () / 2) ; x2 :: new ([W :: unsafe_read_le (input . 0) , W :: unsafe_read_le (input . 1)]) } # [inline (always)] unsafe fn unsafe_read_be (input : & [u8]) -> Self { let input = input . split_at (input . len () / 2) ; x2 :: new ([W :: unsafe_read_be (input . 0) , W :: unsafe_read_be (input . 1)]) } # [inline (always)] fn write_le (self , out : & mut [u8]) { let out = out . split_at_mut (out . len () / 2) ; self . 0 [0] . write_le (out . 0) ; self . 0 [1] . write_le (out . 1) ; } # [inline (always)] fn write_be (self , out : & mut [u8]) { let out = out . split_at_mut (out . len () / 2) ; self . 0 [0] . write_be (out . 0) ; self . 0 [1] . write_be (out . 1) ; } }
    };
}

impl_30!()