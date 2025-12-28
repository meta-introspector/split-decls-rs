macro_rules! deps {
    () => {
        One!();
        NonZero!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < T > ConstOne for NonZero < T > where T : ConstOne + One , { const ONE : Self = Self (T :: ONE) ; }
    };
}

impl_184!()