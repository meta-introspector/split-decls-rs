macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " The Display implementation allows the std::error::Error implementation"] impl < I : fmt :: Display > fmt :: Display for Error < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "error {:?} at: {}" , self . code , self . input) } }
    };
}

impl_14!()