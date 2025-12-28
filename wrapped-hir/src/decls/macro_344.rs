macro_rules! deps {
    () => {
        TypeAlias!();
        Enum!();
        Function!();
        Union!();
        Adt!();
        Struct!();
        Impl!();
        Const!();
        Static!();
        GenericDef!();
        Trait!();
    };
}

macro_rules! macro_344 {
    () => {
        deps!();
        impl_from ! (Function , Adt (Struct , Enum , Union) , Trait , TypeAlias , Impl , Const , Static for GenericDef) ;
    };
}

macro_344!();