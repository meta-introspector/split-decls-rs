macro_rules! deps {
    () => {
        FutureTestExt!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < Fut > FutureTestExt for Fut where Fut : Future { }
    };
}

impl_49!()