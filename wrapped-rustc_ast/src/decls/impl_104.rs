macro_rules! deps {
    () => {
        MacCall!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl MacCall { pub fn span (& self) -> Span { self . path . span . to (self . args . dspan . entire ()) } }
    };
}

impl_104!()