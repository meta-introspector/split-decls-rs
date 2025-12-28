macro_rules! deps {
    () => {
        Struct!();
        Union!();
        Enum!();
        Adt!();
    };
}

macro_rules! macro_58 {
    () => {
        deps!();
        impl_from ! (Struct , Union , Enum for Adt) ;
    };
}

macro_58!()