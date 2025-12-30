// Generated macro for impl_1095 (impl)
macro_rules! Depcrate_test_runner_configimpl_1095 {
() => {
// Module: crate::test_runner::config
// Provides: {"impl_1095"}
// Dependencies: {}
impl fmt :: Display for RngSeed { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RngSeed :: Random => write ! (f , "random") , RngSeed :: Fixed (n) => write ! (f , "{}" , n) , } } }
};
}
