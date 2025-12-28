macro_rules! deps {
    () => {
        Closure!();
        Struct!();
        Variant!();
        Function!();
    };
}

macro_rules! CallableKind {
    () => {
        deps!();
        pub enum CallableKind < 'db > { Function (Function) , TupleStruct (Struct) , TupleEnumVariant (Variant) , Closure (Closure < 'db >) , FnPtr , FnImpl (FnTrait) , }
    };
}

CallableKind!()