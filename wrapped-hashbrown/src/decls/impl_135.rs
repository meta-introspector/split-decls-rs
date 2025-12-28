macro_rules! deps {
    () => {
        ParValues!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < K : Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParValues < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { & x . as_ref () . 1 }) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_135!();