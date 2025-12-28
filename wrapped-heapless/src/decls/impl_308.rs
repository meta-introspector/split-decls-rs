macro_rules! deps {
    () => {
        Vec!();
        IntoIter!();
        LenType!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > Clone for IntoIter < T , N , LenT > where T : Clone , { fn clone (& self) -> Self { let mut vec = Vec :: new () ; if self . next < self . vec . len { let s = unsafe { slice :: from_raw_parts (self . vec . buffer . buffer . as_ptr () . cast :: < T > () . add (self . next . into_usize ()) , (self . vec . len - self . next) . into_usize () ,) } ; vec . extend_from_slice (s) . ok () ; } Self { vec , next : LenT :: ZERO , } } }
    };
}

impl_308!();