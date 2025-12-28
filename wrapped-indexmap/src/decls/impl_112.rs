macro_rules! deps {
    () => {
        ParIterMut!();
        Bucket!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: refs) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_112!();