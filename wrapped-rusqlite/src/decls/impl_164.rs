macro_rules! deps {
    () => {
        Result!();
        Params!();
        ToSql!();
        Statement!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl Params for [& (dyn ToSql + Send + Sync) ; 0] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . ensure_parameter_count (0) } }
    };
}

impl_164!();