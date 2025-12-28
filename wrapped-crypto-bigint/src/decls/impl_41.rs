macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T : ConditionallySelectable > ConditionallySelectable for Checked < T > { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (CtOption :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_41!();