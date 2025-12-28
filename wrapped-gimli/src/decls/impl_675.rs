macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> :: core :: result :: Result < () , fmt :: Error > { write ! (f , "{}" , self . description ()) } }
    };
}

impl_675!();