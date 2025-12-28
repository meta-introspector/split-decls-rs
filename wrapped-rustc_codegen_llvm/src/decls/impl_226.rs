macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl HasDataLayout for CodegenCx < '_ , '_ > { # [inline] fn data_layout (& self) -> & TargetDataLayout { & self . tcx . data_layout } }
    };
}

impl_226!()