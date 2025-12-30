// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl < C > AlgorithmName for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone + AlgorithmName , { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("RetailMac<") ? ; < C as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
};
}
