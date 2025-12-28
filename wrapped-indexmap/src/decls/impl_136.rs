macro_rules! deps {
    () => {
        Bucket!();
        ParValuesMut!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < K , V : fmt :: Debug > fmt :: Debug for ParValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: value_ref) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_136!()