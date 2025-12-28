macro_rules! deps {
    () => {
        ParIter!();
        Bucket!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for ParIter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: refs) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_106!()