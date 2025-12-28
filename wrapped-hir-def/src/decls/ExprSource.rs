macro_rules! deps {
    () => {
        ExprPtr!();
    };
}

macro_rules! ExprSource {
    () => {
        deps!();
        pub type ExprSource = InFile < ExprPtr > ;
    };
}

ExprSource!()