macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Debug for Group { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = fmt . debug_struct ("Group") ; debug . field ("delimiter" , & self . delimiter) ; debug . field ("stream" , & self . stream) ; debug_span_field_if_nontrivial (& mut debug , self . span) ; debug . finish () } }
    };
}

impl_117!();