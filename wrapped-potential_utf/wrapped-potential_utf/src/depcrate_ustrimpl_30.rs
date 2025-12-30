// Generated macro for impl_30 (impl)
macro_rules! Depcrate_ustrimpl_30 {
() => {
// Module: crate::ustr
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Debug for PotentialUtf8 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_as_str () { Ok (s) => fmt :: Debug :: fmt (s , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
};
}
