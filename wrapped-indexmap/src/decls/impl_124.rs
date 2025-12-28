macro_rules! deps {
    () => {
        Bucket!();
        ParKeys!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < K : fmt :: Debug , V > fmt :: Debug for ParKeys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_124!();