macro_rules! deps {
    () => {
        EndsWithNewline!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > + ? Sized > EndsWithNewline for T { fn ends_with_newline (& self) -> bool { self . as_ref () . ends_with (b"\n") } }
    };
}

impl_79!();