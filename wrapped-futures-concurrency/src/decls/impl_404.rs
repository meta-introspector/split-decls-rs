macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < S , const N : usize > fmt :: Debug for Chain < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_404!()