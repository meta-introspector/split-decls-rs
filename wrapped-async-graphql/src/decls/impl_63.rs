macro_rules! deps {
    () => {
        Guard!();
        GuardExt!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : Guard > GuardExt for T { }
    };
}

impl_63!();