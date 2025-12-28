macro_rules! deps {
    () => {
        ScaleFactorTrait!();
        Properties!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl crate :: ScaleFactorTrait for Properties { fn scale_factor (& self) -> f64 { self . scale_factor } }
    };
}

impl_37!()