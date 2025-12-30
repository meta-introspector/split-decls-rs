// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl < C : PmacCipher + AlgorithmName , const LC_SIZE : usize > AlgorithmName for PmacCore < C , LC_SIZE > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Pmac<") ? ; < C as AlgorithmName > :: write_alg_name (f) ? ; f . write_str (">") } }
};
}
