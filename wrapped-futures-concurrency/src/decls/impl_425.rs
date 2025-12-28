macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl < S > fmt :: Debug for Chain < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_425!()