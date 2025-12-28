macro_rules! deps {
    () => {
        Macro!();
        Function!();
        ModuleDef!();
        Adt!();
        Variant!();
        Const!();
        Union!();
        Static!();
        TypeAlias!();
        BuiltinType!();
        Trait!();
        Enum!();
        Module!();
        Struct!();
    };
}

macro_rules! macro_232 {
    () => {
        deps!();
        impl_from ! (Module , Function , Adt (Struct , Enum , Union) , Variant , Const , Static , Trait , TypeAlias , BuiltinType , Macro for ModuleDef) ;
    };
}

macro_232!()