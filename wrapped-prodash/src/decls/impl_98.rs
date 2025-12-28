macro_rules! deps {
    () => {
        Step!();
        DisplayValue!();
        Bytes!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl DisplayValue for Bytes { fn display_current_value (& self , w : & mut dyn fmt :: Write , value : Step , _upper : Option < Step >) -> fmt :: Result { Self :: format_bytes (w , value) } fn display_upper_bound (& self , w : & mut dyn fmt :: Write , upper_bound : Step , _value : Step) -> fmt :: Result { Self :: format_bytes (w , upper_bound) } fn dyn_hash (& self , state : & mut dyn std :: hash :: Hasher) { state . write (& []) } fn display_unit (& self , _w : & mut dyn fmt :: Write , _value : Step) -> fmt :: Result { Ok (()) } }
    };
}

impl_98!()