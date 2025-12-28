macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let desc = self . description () ; match self . extra { Some (ref s) => write ! (f , "[{}] {} ({})" , self . code () , desc , s) , None => write ! (f , "[{}] {}" , self . code () , desc) , } } }
    };
}

impl_2!()