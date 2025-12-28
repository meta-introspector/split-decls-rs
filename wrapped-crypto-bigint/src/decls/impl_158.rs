macro_rules! deps {
    () => {
        Limb!();
        Word!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl ConditionallySelectable for Limb { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (Word :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_158!()