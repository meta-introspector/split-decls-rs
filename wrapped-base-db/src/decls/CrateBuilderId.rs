macro_rules! deps {
    () => {
        CrateBuilder!();
    };
}

macro_rules! CrateBuilderId {
    () => {
        deps!();
        pub type CrateBuilderId = Idx < CrateBuilder > ;
    };
}

CrateBuilderId!()