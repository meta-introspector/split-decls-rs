macro_rules! deps {
    () => {
        TypePtr!();
    };
}

macro_rules! TypeSource {
    () => {
        deps!();
        pub type TypeSource = InFile < TypePtr > ;
    };
}

TypeSource!();