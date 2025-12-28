macro_rules! deps {
    () => {
        InheritableField!();
    };
}

macro_rules! InheritableSemverVersion {
    () => {
        deps!();
        pub type InheritableSemverVersion = InheritableField < semver :: Version > ;
    };
}

InheritableSemverVersion!()