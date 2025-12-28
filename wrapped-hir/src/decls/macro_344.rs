macro_rules! deps {
    () => {
        Struct!();
        Static!();
        GenericDef!();
        Trait!();
        Adt!();
        Enum!();
        Union!();
        TypeAlias!();
        Const!();
        Function!();
        Impl!();
    };
}

macro_rules! macro_344 {
    () => {
        deps!();
        impl_from ! (Function , Adt (Struct , Enum , Union) , Trait , TypeAlias , Impl , Const , Static for GenericDef) ;
    };
}

macro_344!()