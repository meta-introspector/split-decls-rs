// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < C : CmacCipher + AlgorithmName > AlgorithmName for Cmac < C > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }
};
}
