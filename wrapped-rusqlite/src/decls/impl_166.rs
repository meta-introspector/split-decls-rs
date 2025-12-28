macro_rules! deps {
    () => {
        Result!();
        Statement!();
        ToSql!();
        Params!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl Params for & [& dyn ToSql] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters (self) } }
    };
}

impl_166!();