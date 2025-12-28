macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < S > fmt :: Debug for Zip < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
    };
}

impl_501!()