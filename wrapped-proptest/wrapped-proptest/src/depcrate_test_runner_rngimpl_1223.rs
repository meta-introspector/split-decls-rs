// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_test_runner_rngimpl_1223 {
() => {
// Module: crate::test_runner::rng
// Provides: {"impl_1223"}
// Dependencies: {}
impl RngAlgorithm { pub (crate) fn persistence_key (self) -> & 'static str { match self { RngAlgorithm :: XorShift => "xs" , RngAlgorithm :: ChaCha => "cc" , RngAlgorithm :: PassThrough => "pt" , RngAlgorithm :: Recorder => "rc" , RngAlgorithm :: _NonExhaustive => unreachable ! () , } } pub (crate) fn from_persistence_key (k : & str) -> Option < Self > { match k { "xs" => Some (RngAlgorithm :: XorShift) , "cc" => Some (RngAlgorithm :: ChaCha) , "pt" => Some (RngAlgorithm :: PassThrough) , "rc" => Some (RngAlgorithm :: Recorder) , _ => None , } } }
};
}
