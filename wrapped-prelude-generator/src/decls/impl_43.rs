macro_rules! deps {
    () => {
        Declaration!();
        DeclarationInstance!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T : Declaration > Declaration for DeclarationInstance < T > { fn name (& self) -> & str { self . inner . name () } }
    };
}

impl_43!()