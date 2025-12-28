macro_rules! deps {
    () => {
        Declaration!();
        Uses!();
        DeclarationInstance!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T : Declaration , U : Declaration > Uses < U > for DeclarationInstance < T > { fn uses (& self , _other : & U) -> bool { false } }
    };
}

impl_44!();