macro_rules! deps {
    () => {
        EdgeIndex!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        impl < Ix : fmt :: Debug > fmt :: Debug for EdgeIndex < Ix > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "EdgeIndex({:?})" , self . 0) } }
    };
}

impl_664!();