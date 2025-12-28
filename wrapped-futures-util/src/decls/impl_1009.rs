macro_rules! deps {
    () => {
        Future01CompatExt!();
    };
}

macro_rules! impl_1009 {
    () => {
        deps!();
        impl < Fut : Future01 > Future01CompatExt for Fut { }
    };
}

impl_1009!();