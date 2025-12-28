macro_rules! deps {
    () => {
        Enum!();
        Union!();
        Struct!();
        Adt!();
    };
}

macro_rules! macro_274 {
    () => {
        deps!();
        impl_from ! (Struct , Union , Enum for Adt) ;
    };
}

macro_274!();