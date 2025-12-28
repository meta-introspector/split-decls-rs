macro_rules! deps {
    () => {
        Bucket!();
        IntoParIter!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for IntoParIter < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_148!()