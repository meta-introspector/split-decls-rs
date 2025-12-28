macro_rules! deps {
    () => {
        DeclarationInstance!();
        Declaration!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T : Declaration > Declaration for DeclarationInstance < T > { fn name (& self) -> & str { self . inner . name () } }
    };
}

impl_43!();