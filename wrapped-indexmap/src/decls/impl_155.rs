macro_rules! deps {
    () => {
        Bucket!();
        ParIter!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ParIter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = self . entries . iter () . map (Bucket :: key_ref) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_155!()