macro_rules! deps {
    () => {
        DepKind!();
        Package!();
    };
}

macro_rules! PackageDependency {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct PackageDependency { pub pkg : Package , pub name : String , pub kind : DepKind , }
    };
}

PackageDependency!();