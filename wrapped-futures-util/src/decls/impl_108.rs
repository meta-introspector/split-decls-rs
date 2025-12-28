macro_rules! deps {
    () => {
        FutureExt!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T : ? Sized > FutureExt for T where T : Future { }
    };
}

impl_108!()