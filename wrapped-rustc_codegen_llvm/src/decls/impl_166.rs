macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl HasTargetSpec for Builder < '_ , '_ , '_ > { # [inline] fn target_spec (& self) -> & Target { self . cx . target_spec () } }
    };
}

impl_166!()