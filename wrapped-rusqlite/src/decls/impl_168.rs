macro_rules! deps {
    () => {
        Statement!();
        Params!();
        BindIndex!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < S : BindIndex , T : ToSql > Params for & [(S , T)] { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters_named (self) } }
    };
}

impl_168!()