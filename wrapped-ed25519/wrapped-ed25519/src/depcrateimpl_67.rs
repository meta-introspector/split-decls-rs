// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Debug for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ed25519::Signature") . field ("R" , & hex :: ComponentFormatter (self . r_bytes ())) . field ("s" , & hex :: ComponentFormatter (self . s_bytes ())) . finish () } }
};
}
