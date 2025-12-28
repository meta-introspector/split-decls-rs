macro_rules! deps {
    () => {
        Formatter!();
        Result!();
        ErrorImpl!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Display for ErrorImpl { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . line == 0 { Display :: fmt (& self . code , f) } else { write ! (f , "{} at line {} column {}" , self . code , self . line , self . column) } } }
    };
}

impl_66!()