macro_rules! deps {
    () => {
        ImplTraitLoweringState!();
        ImplTraitLoweringMode!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'db > ImplTraitLoweringState < 'db > { fn new (mode : ImplTraitLoweringMode) -> ImplTraitLoweringState < 'db > { Self { mode , opaque_type_data : Arena :: new () } } }
    };
}

impl_112!()