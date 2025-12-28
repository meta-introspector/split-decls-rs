macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl FieldAttributes { pub const Literal : Self = Self (0x40) ; }
    };
}

impl_336!()