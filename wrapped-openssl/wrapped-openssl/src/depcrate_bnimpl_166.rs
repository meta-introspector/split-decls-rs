// Generated macro for impl_166 (impl)
macro_rules! Depcrate_bnimpl_166 {
() => {
// Module: crate::bn
// Provides: {"impl_166"}
// Dependencies: {}
impl fmt :: Display for BigNum { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . to_dec_str () { Ok (s) => f . write_str (& s) , Err (e) => Err (e . into ()) , } } }
};
}
