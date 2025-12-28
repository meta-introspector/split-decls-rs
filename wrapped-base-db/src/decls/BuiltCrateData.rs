macro_rules! deps {
    () => {
        CrateData!();
        Crate!();
    };
}

macro_rules! BuiltCrateData {
    () => {
        deps!();
        pub type BuiltCrateData = CrateData < Crate > ;
    };
}

BuiltCrateData!()