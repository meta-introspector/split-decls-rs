// Generated macro for impl_163 (impl)
macro_rules! Depcrate_bnimpl_163 {
() => {
// Module: crate::bn
// Provides: {"impl_163"}
// Dependencies: {}
impl fmt :: Debug for BigNumRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . to_dec_str () { Ok (s) => f . write_str (& s) , Err (e) => Err (e . into ()) , } } }
};
}
