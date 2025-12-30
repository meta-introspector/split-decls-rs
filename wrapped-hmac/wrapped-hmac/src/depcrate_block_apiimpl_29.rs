// Generated macro for impl_29 (impl)
macro_rules! Depcrate_block_apiimpl_29 {
() => {
// Module: crate::block_api
// Provides: {"impl_29"}
// Dependencies: {}
impl < D : EagerHash + AlgorithmName > AlgorithmName for HmacResetCore < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Hmac<") ? ; < D as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
};
}
