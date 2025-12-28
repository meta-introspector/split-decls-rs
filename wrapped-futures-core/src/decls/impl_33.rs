macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl fmt :: Debug for AtomicWaker { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "AtomicWaker") } }
    };
}

impl_33!();