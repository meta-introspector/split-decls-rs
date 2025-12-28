macro_rules! deps {
    () => {
        Executor01As03!();
        Executor01Future!();
        Executor01CompatExt!();
        Send!();
    };
}

macro_rules! impl_1000 {
    () => {
        deps!();
        impl < Ex > Executor01CompatExt for Ex where Ex : Executor01 < Executor01Future > + Clone + Send + 'static , { fn compat (self) -> Executor01As03 < Self > { Executor01As03 { executor01 : self } } }
    };
}

impl_1000!()