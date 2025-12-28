macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Debug for PotentialCodePoint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_to_char () { Ok (c) => fmt :: Debug :: fmt (& c , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
    };
}

impl_5!()