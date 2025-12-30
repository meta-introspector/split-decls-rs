// Generated macro for impl_125 (impl)
macro_rules! Depcrate_hashimpl_125 {
() => {
// Module: crate::hash
// Provides: {"impl_125"}
// Dependencies: {}
impl fmt :: Display for Hash { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , bs58 :: encode (self . 0) . into_string ()) } }
};
}
