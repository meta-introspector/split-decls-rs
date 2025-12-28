macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ParIter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { x . as_ref () }) ; f . debug_list () . entries (iter) . finish () } }
    };
}

impl_207!()