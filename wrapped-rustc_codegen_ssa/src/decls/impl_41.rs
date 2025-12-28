macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl fmt :: Debug for Command { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . command () . fmt (f) } }
    };
}

impl_41!()