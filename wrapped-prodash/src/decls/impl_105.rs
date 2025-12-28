macro_rules! deps {
    () => {
        DisplayValue!();
        Step!();
        Human!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl DisplayValue for Human { fn display_current_value (& self , w : & mut dyn fmt :: Write , value : Step , _upper : Option < Step >) -> fmt :: Result { self . format_bytes (w , value) } fn display_upper_bound (& self , w : & mut dyn fmt :: Write , upper_bound : Step , _value : Step) -> fmt :: Result { self . format_bytes (w , upper_bound) } fn dyn_hash (& self , state : & mut dyn Hasher) { state . write (self . name . as_bytes ()) ; state . write_u8 (0) ; } fn display_unit (& self , w : & mut dyn fmt :: Write , _value : Step) -> fmt :: Result { w . write_str (self . name) } }
    };
}

impl_105!();