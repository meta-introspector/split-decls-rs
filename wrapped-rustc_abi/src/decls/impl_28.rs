macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Debug for Size { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Size({} bytes)" , self . bytes ()) } }
    };
}

impl_28!()