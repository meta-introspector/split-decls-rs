macro_rules! deps {
    () => {
        NonZeroChar!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl NonZeroChar { pub fn new (ch : char) -> Option < Self > { if ch == '\0' { None } else { Some (NonZeroChar (ch)) } } pub fn get (self) -> char { self . 0 } }
    };
}

impl_151!()