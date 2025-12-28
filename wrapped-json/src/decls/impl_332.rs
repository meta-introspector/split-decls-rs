macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl PartialEq < str > for Value { fn eq (& self , other : & str) -> bool { eq_str (self , other) } }
    };
}

impl_332!()