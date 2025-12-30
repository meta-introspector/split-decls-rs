// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < C > AlgorithmName for CbcMac < C > where C : BlockCipherEncrypt + Clone + AlgorithmName , { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }
};
}
