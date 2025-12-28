macro_rules! deps {
    () => {
        Flags!();
        IterNames!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < B : Flags > IterNames < B > { pub (crate) fn new (flags : & B) -> Self { IterNames { flags : B :: FLAGS , idx : 0 , remaining : B :: from_bits_retain (flags . bits ()) , source : B :: from_bits_retain (flags . bits ()) , } } }
    };
}

impl_5!()