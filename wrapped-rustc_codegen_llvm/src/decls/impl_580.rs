macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_580 {
    () => {
        deps!();
        impl < 'll > CodegenCx < 'll , '_ > { }
    };
}

impl_580!();