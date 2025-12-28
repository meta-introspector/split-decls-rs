macro_rules! deps {
    () => {
        Result!();
        DependencyKind!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Display for DependencyKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = serde_json :: to_string (self) . unwrap () ; f . write_str (& s [1 .. s . len () - 1]) } }
    };
}

impl_1!();