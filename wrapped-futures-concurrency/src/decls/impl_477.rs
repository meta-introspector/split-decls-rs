macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        impl < S , const N : usize > fmt :: Debug for Zip < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_477!()