// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl < D : EagerHash + AlgorithmName > AlgorithmName for Hmac < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }
};
}
