macro_rules! deps {
    () => {
        ConstantTimeSelect!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < T : ConditionallySelectable > ConstantTimeSelect for T { # [inline (always)] fn ct_select (a : & Self , b : & Self , choice : Choice) -> Self { T :: conditional_select (a , b , choice) } # [inline (always)] fn ct_assign (& mut self , other : & Self , choice : Choice) { self . conditional_assign (other , choice) } # [inline (always)] fn ct_swap (a : & mut Self , b : & mut Self , choice : Choice) { T :: conditional_swap (a , b , choice) } }
    };
}

impl_268!()