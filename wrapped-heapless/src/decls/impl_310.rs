macro_rules! deps {
    () => {
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > Drop for IntoIter < T , N , LenT > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (& mut self . vec . as_mut_slice () [self . next . into_usize () ..]) ; self . vec . len = LenT :: ZERO ; } } }
    };
}

impl_310!();