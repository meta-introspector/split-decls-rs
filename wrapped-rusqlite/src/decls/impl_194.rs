macro_rules! deps {
    () => {
        Statement!();
        ToSql!();
        ParamsFromIter!();
        Params!();
        Result!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < I > Params for ParamsFromIter < I > where I : IntoIterator , I :: Item : ToSql , { # [inline] fn __bind_in (self , stmt : & mut Statement < '_ >) -> Result < () > { stmt . bind_parameters (self . 0) } }
    };
}

impl_194!();