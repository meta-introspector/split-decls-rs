macro_rules! deps {
    () => {
        Struct!();
        Function!();
        Variant!();
        Closure!();
    };
}

macro_rules! CallableKind {
    () => {
        deps!();
        pub enum CallableKind < 'db > { Function (Function) , TupleStruct (Struct) , TupleEnumVariant (Variant) , Closure (Closure < 'db >) , FnPtr , FnImpl (FnTrait) , }
    };
}

CallableKind!();