macro_rules! deps {
    () => {
        ReadRef!();
        Comdat!();
        Result!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Comdat < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("Comdat") ; s . field ("symbol" , & self . symbol ()) . field ("name" , & self . name () . unwrap_or ("<invalid>")) . field ("kind" , & self . kind ()) . finish () } }
    };
}

impl_148!();