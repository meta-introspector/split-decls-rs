// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl < C > fmt :: Debug for CbcMacCore < C > where C : BlockCipherEncrypt + Clone + AlgorithmName , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("CbcMacCore<") ? ; < C as AlgorithmName > :: write_alg_name (f) ? ; f . write_str ("> { ... }") } }
};
}
