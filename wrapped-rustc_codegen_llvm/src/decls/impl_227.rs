macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl HasTargetSpec for CodegenCx < '_ , '_ > { # [inline] fn target_spec (& self) -> & Target { & self . tcx . sess . target } }
    };
}

impl_227!()