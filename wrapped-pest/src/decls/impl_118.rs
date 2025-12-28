macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl fmt :: Debug for Position < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Position") . field ("pos" , & self . pos) . finish () } }
    };
}

impl_118!()