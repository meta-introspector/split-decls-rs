macro_rules! deps {
    () => {
        Idx!();
        ChunkedBitSet!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T : Idx > fmt :: Debug for ChunkedBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_36!()