macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < T > ConditionallySelectable for Odd < T > where T : ConditionallySelectable , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_231!();