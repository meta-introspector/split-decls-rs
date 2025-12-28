macro_rules! deps {
    () => {
        ScalarRange!();
        Result!();
        Formatter!();
    };
}

macro_rules! impl_871 {
    () => {
        deps!();
        impl fmt :: Debug for ScalarRange { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ScalarRange({:X}, {:X})" , self . start , self . end) } }
    };
}

impl_871!()