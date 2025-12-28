macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T , N : ArrayLength > ConditionallySelectable for GenericArray < T , N > where GenericArray < T , N > : Copy , T : ConditionallySelectable , { # [inline] fn conditional_select (a : & Self , b : & Self , choice : subtle :: Choice) -> Self { let mut out = * a ; out . conditional_assign (b , choice) ; out } # [inline] fn conditional_assign (& mut self , other : & Self , choice : subtle :: Choice) { for (a , b) in self . iter_mut () . zip (other . iter ()) { a . conditional_assign (b , choice) ; } } }
    };
}

impl_88!()