macro_rules! deps {
    () => {
        Function!();
        Trait!();
        Impl!();
        Static!();
        Enum!();
        Adt!();
        TypeAlias!();
        Struct!();
        Union!();
        GenericDef!();
        Const!();
    };
}

macro_rules! macro_128 {
    () => {
        deps!();
        impl_from ! (Function , Adt (Struct , Enum , Union) , Trait , TypeAlias , Impl , Const , Static for GenericDef) ;
    };
}

macro_128!()