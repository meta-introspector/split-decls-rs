macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Interned < T > { # [cold] fn drop_slow (& mut self) { let storage = T :: storage () . get () ; if Arc :: count (& self . arc) != 2 { return ; } storage . remove (& self . arc) ; } }
    };
}

impl_7!()