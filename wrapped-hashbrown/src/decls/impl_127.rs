macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParIter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { let r = x . as_ref () ; (& r . 0 , & r . 1) }) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_127!()