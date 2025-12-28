macro_rules! deps {
    () => {
        Merge!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < S , const N : usize > fmt :: Debug for Merge < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_436!()