// Generated macro for impl_230 (impl)
macro_rules! Depcrateimpl_230 {
() => {
// Module: crate
// Provides: {"impl_230"}
// Dependencies: {}
impl fmt :: Debug for Hasher { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Hasher") . field ("flags" , & self . chunk_state . flags) . field ("platform" , & self . chunk_state . platform) . finish () } }
};
}
