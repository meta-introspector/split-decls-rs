macro_rules! deps {
    () => {
        IterNames!();
        Flags!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < B : Flags > IterNames < B > { pub (crate) fn new (flags : & B) -> Self { IterNames { flags : B :: FLAGS , idx : 0 , remaining : B :: from_bits_retain (flags . bits ()) , source : B :: from_bits_retain (flags . bits ()) , } } }
    };
}

impl_105!();