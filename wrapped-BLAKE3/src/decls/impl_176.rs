macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        # [doc = " This implementation is constant-time if the target is 32 bytes long."] impl PartialEq < [u8] > for Hash { # [inline] fn eq (& self , other : & [u8]) -> bool { constant_time_eq :: constant_time_eq (& self . 0 , other) } }
    };
}

impl_176!();