macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl Display for Target { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , Self :: name (* self)) } }
    };
}

impl_472!()