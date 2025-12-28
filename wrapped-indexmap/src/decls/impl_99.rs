macro_rules! deps {
    () => {
        Bucket!();
        IntoParIter!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for IntoParIter < K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: refs) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_99!();