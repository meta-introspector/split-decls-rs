macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > HasDataLayout for InterpCx < 'tcx , M > { # [inline] fn data_layout (& self) -> & TargetDataLayout { & self . tcx . data_layout } }
    };
}

impl_207!();