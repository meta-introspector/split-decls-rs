macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl Debug for Literal { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = fmt . debug_struct ("Literal") ; debug . field ("lit" , & format_args ! ("{}" , self . repr)) ; debug_span_field_if_nontrivial (& mut debug , self . span) ; debug . finish () } }
    };
}

impl_133!();