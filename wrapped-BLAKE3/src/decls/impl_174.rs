macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        # [doc = " This implementation is constant-time."] impl PartialEq for Hash { # [inline] fn eq (& self , other : & Hash) -> bool { constant_time_eq :: constant_time_eq_32 (& self . 0 , & other . 0) } }
    };
}

impl_174!()