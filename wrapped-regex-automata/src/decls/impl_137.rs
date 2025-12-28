macro_rules! deps {
    () => {
        StartStateIter!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a , T > fmt :: Debug for StartStateIter < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("StartStateIter") . field ("i" , & self . i) . finish () } }
    };
}

impl_137!()