macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl PartialEq < String > for Value { fn eq (& self , other : & String) -> bool { eq_str (self , other . as_str ()) } }
    };
}

impl_336!();