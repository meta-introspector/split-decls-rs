macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl PartialEq < Value > for String { fn eq (& self , other : & Value) -> bool { eq_str (other , self . as_str ()) } }
    };
}

impl_337!();