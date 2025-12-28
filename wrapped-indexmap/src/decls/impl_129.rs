macro_rules! deps {
    () => {
        ParValues!();
        Bucket!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < K , V : fmt :: Debug > fmt :: Debug for ParValues < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: value_ref) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_129!()