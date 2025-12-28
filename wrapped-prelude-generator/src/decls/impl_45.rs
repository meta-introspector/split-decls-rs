macro_rules! deps {
    () => {
        Is!();
        DeclarationInstance!();
        Declaration!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T : Declaration > Is < T > for DeclarationInstance < T > { fn is_type (& self) -> bool { true } }
    };
}

impl_45!()