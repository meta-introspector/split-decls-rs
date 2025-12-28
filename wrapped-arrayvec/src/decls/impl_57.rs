macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T , const CAP : usize > fmt :: Debug for IntoIter < T , CAP > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (& self . v [self . index ..]) . finish () } }
    };
}

impl_57!()