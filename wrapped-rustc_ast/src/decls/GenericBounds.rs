macro_rules! deps {
    () => {
        GenericBound!();
    };
}

macro_rules! GenericBounds {
    () => {
        deps!();
        pub type GenericBounds = Vec < GenericBound > ;
    };
}

GenericBounds!()