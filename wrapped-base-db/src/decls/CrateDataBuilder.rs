macro_rules! deps {
    () => {
        CrateData!();
        CrateBuilderId!();
    };
}

macro_rules! CrateDataBuilder {
    () => {
        deps!();
        pub type CrateDataBuilder = CrateData < CrateBuilderId > ;
    };
}

CrateDataBuilder!()