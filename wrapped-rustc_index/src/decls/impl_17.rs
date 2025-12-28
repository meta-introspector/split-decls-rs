macro_rules! deps {
    () => {
        DenseBitSet!();
        Idx!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : Idx > fmt :: Debug for DenseBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_17!()