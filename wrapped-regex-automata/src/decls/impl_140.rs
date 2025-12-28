macro_rules! deps {
    () => {
        StateIter!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'a , T > fmt :: Debug for StateIter < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("StateIter") . field ("id" , & self . id) . finish () } }
    };
}

impl_140!();