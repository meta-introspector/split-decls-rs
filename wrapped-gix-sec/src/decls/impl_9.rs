macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < R > Display for Error < R > where R : std :: fmt :: Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Not allowed to handle resource {:?}: permission denied" , self . resource) } }
    };
}

impl_9!()