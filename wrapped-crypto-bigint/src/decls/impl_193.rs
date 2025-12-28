macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < T > ConditionallySelectable for NonZero < T > where T : ConditionallySelectable , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_193!();