macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'tcx > abi :: HasDataLayout for LateContext < 'tcx > { # [inline] fn data_layout (& self) -> & abi :: TargetDataLayout { & self . tcx . data_layout } }
    };
}

impl_130!();