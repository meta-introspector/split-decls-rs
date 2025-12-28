macro_rules! deps {
    () => {
        Union!();
        Enum!();
        Struct!();
        Adt!();
    };
}

macro_rules! macro_5 {
    () => {
        deps!();
        impl_has_attrs_enum ! [Struct , Union , Enum for Adt] ;
    };
}

macro_5!();