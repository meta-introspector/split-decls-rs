macro_rules! deps {
    () => {
        ParKeys!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < K : fmt :: Debug + Eq + Hash , V > fmt :: Debug for ParKeys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { & x . as_ref () . 0 }) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_131!()