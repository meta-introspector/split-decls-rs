macro_rules! deps {
    () => {
        CrateDisplayName!();
        CrateBuilderId!();
        CyclicDependenciesError!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl CyclicDependenciesError { fn from (& self) -> & (CrateBuilderId , Option < CrateDisplayName >) { self . path . first () . unwrap () } fn to (& self) -> & (CrateBuilderId , Option < CrateDisplayName >) { self . path . last () . unwrap () } }
    };
}

impl_60!()