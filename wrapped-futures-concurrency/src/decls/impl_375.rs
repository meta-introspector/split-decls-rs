macro_rules! deps {
    () => {
        TryJoin!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl < Fut , T , E > fmt :: Debug for TryJoin < Fut , T , E > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
    };
}

impl_375!();