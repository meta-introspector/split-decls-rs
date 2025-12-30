// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl fmt :: Debug for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ed448::Signature") . field ("R" , self . r_bytes ()) . field ("s" , self . s_bytes ()) . finish () } }
};
}
