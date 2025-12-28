macro_rules! deps {
    () => {
        ExprOrPatPtr!();
    };
}

macro_rules! ExprOrPatSource {
    () => {
        deps!();
        pub type ExprOrPatSource = InFile < ExprOrPatPtr > ;
    };
}

ExprOrPatSource!()