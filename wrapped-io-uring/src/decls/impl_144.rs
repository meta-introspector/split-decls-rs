macro_rules! deps {
    () => {
        Entry128!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl private :: Sealed for Entry128 { }
    };
}

impl_144!();