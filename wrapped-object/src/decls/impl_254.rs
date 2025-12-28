macro_rules! deps {
    () => {
        CoffRelocationIterator!();
        Result!();
        ReadRef!();
        CoffHeader!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > fmt :: Debug for CoffRelocationIterator < 'data , 'file , R , Coff > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CoffRelocationIterator") . finish () } }
    };
}

impl_254!();