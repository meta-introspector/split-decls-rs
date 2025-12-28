macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl PartialEq < rt :: rwlock :: Action > for Action { fn eq (& self , other : & rt :: rwlock :: Action) -> bool { let other : Action = (* other) . into () ; * self == other } }
    };
}

impl_103!();