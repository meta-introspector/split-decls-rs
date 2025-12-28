macro_rules! deps {
    () => {
        SelectedOperation!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl fmt :: Debug for SelectedOperation < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("SelectedOperation { .. }") } }
    };
}

impl_185!();