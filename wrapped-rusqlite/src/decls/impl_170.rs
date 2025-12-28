macro_rules! deps {
    () => {
        Result!();
        Params!();
        Statement!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl Params for () { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . ensure_parameter_count (0) } }
    };
}

impl_170!();