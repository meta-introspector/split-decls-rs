// Generated macro for impl_367 (impl)
macro_rules! Depcrate_vecimpl_367 {
() => {
// Module: crate::vec
// Provides: {"impl_367"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > core :: fmt :: Debug for IntoIter < T , N , LenT > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let s = if self . next < self . vec . len { unsafe { slice :: from_raw_parts (self . vec . buffer . buffer . as_ptr () . cast :: < T > () . add (self . next . into_usize ()) , (self . vec . len - self . next) . into_usize () ,) } } else { & [] } ; write ! (f , "{s:?}") } }
};
}
