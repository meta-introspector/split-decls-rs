macro_rules! unknown_const_as_generic {
    () => {
        pub fn unknown_const_as_generic < 'db > (ty : Ty < 'db >) -> GenericArg < 'db > { unknown_const (ty) . into () }
    };
}

unknown_const_as_generic!()