macro_rules! deps {
    () => {
        PatId!();
        ExprOrPatId!();
        ExprId!();
    };
}

macro_rules! macro_216 {
    () => {
        deps!();
        stdx :: impl_from ! (ExprId , PatId for ExprOrPatId) ;
    };
}

macro_216!();