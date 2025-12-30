// Generated macro for impl_164 (impl)
macro_rules! Depcrate_bnimpl_164 {
() => {
// Module: crate::bn
// Provides: {"impl_164"}
// Dependencies: {}
impl fmt :: Debug for BigNum { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . to_dec_str () { Ok (s) => f . write_str (& s) , Err (e) => Err (e . into ()) , } } }
};
}
