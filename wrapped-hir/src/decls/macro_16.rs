macro_rules! deps {
    () => {
        Struct!();
        BuiltinType!();
        Module!();
        Function!();
        Enum!();
        Static!();
        Adt!();
        Macro!();
        ModuleDef!();
        TypeAlias!();
        Const!();
        Variant!();
        Trait!();
        Union!();
    };
}

macro_rules! macro_16 {
    () => {
        deps!();
        impl_from ! (Module , Function , Adt (Struct , Enum , Union) , Variant , Const , Static , Trait , TypeAlias , BuiltinType , Macro for ModuleDef) ;
    };
}

macro_16!()