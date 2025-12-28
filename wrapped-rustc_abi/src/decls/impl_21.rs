macro_rules! deps {
    () => {
        HasDataLayout!();
        TargetDataLayout!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl HasDataLayout for & TargetDataLayout { # [inline] fn data_layout (& self) -> & TargetDataLayout { (* * self) . data_layout () } }
    };
}

impl_21!()