macro_rules! deps {
    () => {
        DisplayValue!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl DisplayValue for & 'static str { fn dyn_hash (& self , state : & mut dyn Hasher) { state . write (self . as_bytes ()) } fn display_unit (& self , w : & mut dyn fmt :: Write , _value : usize) -> fmt :: Result { w . write_fmt (format_args ! ("{}" , self)) } }
    };
}

impl_112!()