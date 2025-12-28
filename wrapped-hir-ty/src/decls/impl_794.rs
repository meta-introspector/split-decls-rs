macro_rules! deps {
    () => {
        ImplTraitLoweringMode!();
        ImplTraitLoweringState!();
    };
}

macro_rules! impl_794 {
    () => {
        deps!();
        impl < 'db > ImplTraitLoweringState < 'db > { fn new (mode : ImplTraitLoweringMode) -> ImplTraitLoweringState < 'db > { Self { mode , opaque_type_data : Arena :: new () } } }
    };
}

impl_794!();