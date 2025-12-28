macro_rules! deps {
    () => {
        PackageData!();
    };
}

macro_rules! Package {
    () => {
        deps!();
        pub type Package = Idx < PackageData > ;
    };
}

Package!()