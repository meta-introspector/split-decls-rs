macro_rules! deps {
    () => {
        HasDataLayout!();
        TargetDataLayout!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl HasDataLayout for TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { self } }
    };
}

impl_20!()