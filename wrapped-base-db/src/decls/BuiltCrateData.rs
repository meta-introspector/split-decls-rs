macro_rules! deps {
    () => {
        Crate!();
        CrateData!();
    };
}

macro_rules! BuiltCrateData {
    () => {
        deps!();
        pub type BuiltCrateData = CrateData < Crate > ;
    };
}

BuiltCrateData!();