macro_rules! deps {
    () => {
        Pos!();
        Result!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl fmt :: Display for Pos { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}:{}" , self . line , self . column) } }
    };
}

impl_119!()