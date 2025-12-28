macro_rules! deps {
    () => {
        Module!();
        Adt!();
        Enum!();
        Static!();
        Const!();
        Variant!();
        Union!();
        Function!();
        TypeAlias!();
        Struct!();
        Trait!();
        BuiltinType!();
        Macro!();
        ModuleDef!();
    };
}

macro_rules! macro_232 {
    () => {
        deps!();
        impl_from ! (Module , Function , Adt (Struct , Enum , Union) , Variant , Const , Static , Trait , TypeAlias , BuiltinType , Macro for ModuleDef) ;
    };
}

macro_232!();