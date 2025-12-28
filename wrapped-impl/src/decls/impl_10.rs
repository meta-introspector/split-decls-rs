macro_rules! deps {
    () => {
        Field!();
        ParamsInScope!();
        Variant!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Variant < 'a > { fn from_syn (node : & 'a syn :: Variant , scope : & ParamsInScope < 'a >) -> Result < Self > { let attrs = attr :: get (& node . attrs) ? ; Ok (Variant { original : node , attrs , ident : node . ident . clone () , fields : Field :: multiple_from_syn (& node . fields , scope) ? , }) } }
    };
}

impl_10!();