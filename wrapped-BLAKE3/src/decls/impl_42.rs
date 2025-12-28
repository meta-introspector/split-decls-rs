macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        # [doc = " This implementation is constant-time."] impl PartialEq < [u8 ; OUT_LEN] > for Hash { # [inline] fn eq (& self , other : & [u8 ; OUT_LEN]) -> bool { constant_time_eq :: constant_time_eq_32 (& self . 0 , other) } }
    };
}

impl_42!()