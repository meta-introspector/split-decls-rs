macro_rules! deps {
    () => {
        Reverse!();
        ConstraintGraph!();
    };
}

macro_rules! ReverseConstraintGraph {
    () => {
        deps!();
        pub (crate) type ReverseConstraintGraph = ConstraintGraph < Reverse > ;
    };
}

ReverseConstraintGraph!();