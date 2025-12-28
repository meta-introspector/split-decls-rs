macro_rules! deps {
    () => {
        Function!();
        Struct!();
        Closure!();
        Variant!();
    };
}

macro_rules! CallableKind {
    () => {
        deps!();
        pub enum CallableKind < 'db > { Function (Function) , TupleStruct (Struct) , TupleEnumVariant (Variant) , Closure (Closure < 'db >) , FnPtr , FnImpl (FnTrait) , }
    };
}

CallableKind!()