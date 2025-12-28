macro_rules! deps {
    () => {
        One!();
        Odd!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < T > ConstOne for Odd < T > where T : ConstOne + One , { const ONE : Self = Self (T :: ONE) ; }
    };
}

impl_234!();