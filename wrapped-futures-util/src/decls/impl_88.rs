macro_rules! deps {
    () => {
        WeakShared!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < Fut : Future > fmt :: Debug for WeakShared < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WeakShared") . finish () } }
    };
}

impl_88!();