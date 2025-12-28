macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl PartialEq < Value > for & str { fn eq (& self , other : & Value) -> bool { eq_str (other , * self) } }
    };
}

impl_335!()