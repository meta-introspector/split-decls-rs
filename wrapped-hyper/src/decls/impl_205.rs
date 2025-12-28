macro_rules! deps {
    () => {
        ServiceFn!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < F , R > Copy for ServiceFn < F , R > where F : Copy { }
    };
}

impl_205!()