// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < C : PmacCipher + AlgorithmName > AlgorithmName for Pmac < C > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }
};
}
