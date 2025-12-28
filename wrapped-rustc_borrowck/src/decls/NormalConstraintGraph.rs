macro_rules! deps {
    () => {
        Normal!();
        ConstraintGraph!();
    };
}

macro_rules! NormalConstraintGraph {
    () => {
        deps!();
        pub (crate) type NormalConstraintGraph = ConstraintGraph < Normal > ;
    };
}

NormalConstraintGraph!();