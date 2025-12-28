macro_rules! deps {
    () => {
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > core :: fmt :: Debug for IntoIter < T , N , LenT > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let s = if self . next < self . vec . len { unsafe { slice :: from_raw_parts (self . vec . buffer . buffer . as_ptr () . cast :: < T > () . add (self . next . into_usize ()) , (self . vec . len - self . next) . into_usize () ,) } } else { & [] } ; write ! (f , "{s:?}") } }
    };
}

impl_309!();