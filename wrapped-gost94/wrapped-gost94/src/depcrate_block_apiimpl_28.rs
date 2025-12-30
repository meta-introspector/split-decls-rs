// Generated macro for impl_28 (impl)
macro_rules! Depcrate_block_apiimpl_28 {
() => {
// Module: crate::block_api
// Provides: {"impl_28"}
// Dependencies: {}
impl < P : Gost94Params > AlgorithmName for Gost94Core < P > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (P :: NAME) } }
};
}
