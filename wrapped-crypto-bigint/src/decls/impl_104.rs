macro_rules! deps {
    () => {
        Uint!();
        Int!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConditionallySelectable for Int < LIMBS > { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (Uint :: conditional_select (& a . 0 , & b . 0 , choice)) } }
    };
}

impl_104!()