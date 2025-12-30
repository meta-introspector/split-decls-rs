// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < D : EagerHash + AlgorithmName > AlgorithmName for HmacCore < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Hmac<") ? ; < D as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
};
}
