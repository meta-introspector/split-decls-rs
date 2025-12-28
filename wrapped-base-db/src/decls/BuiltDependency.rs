macro_rules! deps {
    () => {
        Crate!();
        Dependency!();
    };
}

macro_rules! BuiltDependency {
    () => {
        deps!();
        pub type BuiltDependency = Dependency < Crate > ;
    };
}

BuiltDependency!();