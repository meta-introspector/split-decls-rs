macro_rules! deps {
    () => {
        TargetDataLayout!();
        HasDataLayout!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl HasDataLayout for & TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { (* * self) . data_layout () } }
    };
}

impl_62!();