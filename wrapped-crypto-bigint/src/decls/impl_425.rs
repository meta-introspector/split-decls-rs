macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl < T : ConditionallySelectable > ConditionallySelectable for Wrapping < T > { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Wrapping (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_425!()