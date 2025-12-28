macro_rules! deps {
    () => {
        MirSpan!();
    };
}

macro_rules! macro_905 {
    () => {
        deps!();
        impl_from ! (ExprId , PatId for MirSpan) ;
    };
}

macro_905!()