macro_rules! deps {
    () => {
        Deferred!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl fmt :: Debug for Deferred { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . pad ("Deferred { .. }") } }
    };
}

impl_84!()