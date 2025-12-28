macro_rules! deps {
    () => {
        Formatter!();
        Result!();
        Position!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Position { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Position(o: {:?}, l: {:?}, c: {:?})" , self . offset , self . line , self . column) } }
    };
}

impl_57!()