macro_rules! deps {
    () => {
        ConstraintGraph!();
        Normal!();
    };
}

macro_rules! NormalConstraintGraph {
    () => {
        deps!();
        pub (crate) type NormalConstraintGraph = ConstraintGraph < Normal > ;
    };
}

NormalConstraintGraph!()