macro_rules! deps {
    () => {
        Formatter!();
        Utf8Range!();
        Result!();
    };
}

macro_rules! impl_867 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Range { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == self . end { write ! (f , "[{:X}]" , self . start) } else { write ! (f , "[{:X}-{:X}]" , self . start , self . end) } } }
    };
}

impl_867!();