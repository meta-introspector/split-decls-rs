// Generated macro for impl_165 (impl)
macro_rules! Depcrate_bnimpl_165 {
() => {
// Module: crate::bn
// Provides: {"impl_165"}
// Dependencies: {}
impl fmt :: Display for BigNumRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . to_dec_str () { Ok (s) => f . write_str (& s) , Err (e) => Err (e . into ()) , } } }
};
}
