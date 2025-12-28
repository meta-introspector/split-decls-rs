macro_rules! deps {
    () => {
        Merge!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < S > fmt :: Debug for Merge < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_460!()