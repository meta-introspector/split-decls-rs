macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        # [allow (clippy :: missing_fields_in_debug)] impl Debug for Ident { # [cfg (not (span_locations))] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = f . debug_tuple ("Ident") ; debug . field (& format_args ! ("{}" , self)) ; debug . finish () } # [cfg (span_locations)] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = f . debug_struct ("Ident") ; debug . field ("sym" , & format_args ! ("{}" , self)) ; debug_span_field_if_nontrivial (& mut debug , self . span) ; debug . finish () } }
    };
}

impl_127!()