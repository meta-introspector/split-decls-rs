macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        Statement!();
        Params!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T : ToSql > Params for (T ,) { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . ensure_parameter_count (1) ? ; stmt . raw_bind_parameter (1 , self . 0) ? ; Ok (()) } }
    };
}

impl_172!()