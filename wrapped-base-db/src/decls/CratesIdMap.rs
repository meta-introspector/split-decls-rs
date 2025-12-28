macro_rules! deps {
    () => {
        Crate!();
        CrateBuilderId!();
    };
}

macro_rules! CratesIdMap {
    () => {
        deps!();
        pub type CratesIdMap = FxHashMap < CrateBuilderId , Crate > ;
    };
}

CratesIdMap!()