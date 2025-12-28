macro_rules! deps {
    () => {
        HasDataLayout!();
        TargetDataLayout!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl HasDataLayout for TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { self } }
    };
}

impl_61!()