macro_rules! deps {
    () => {
        MapSpecialCaseFnInto!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < U > std :: fmt :: Debug for MapSpecialCaseFnInto < U > { debug_fmt_fields ! (MapSpecialCaseFnInto , 0) ; }
    };
}

impl_56!();