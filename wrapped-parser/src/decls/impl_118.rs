macro_rules! deps {
    () => {
        Result!();
        Pos!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl fmt :: Debug for Pos { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Pos({}:{})" , self . line , self . column) } }
    };
}

impl_118!()