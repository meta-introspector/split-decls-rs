macro_rules! deps {
    () => {
        CrateBuilderId!();
        Dependency!();
    };
}

macro_rules! DependencyBuilder {
    () => {
        deps!();
        pub type DependencyBuilder = Dependency < CrateBuilderId > ;
    };
}

DependencyBuilder!();