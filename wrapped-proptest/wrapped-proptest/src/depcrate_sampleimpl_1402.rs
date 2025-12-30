// Generated macro for impl_1402 (impl)
macro_rules! Depcrate_sampleimpl_1402 {
() => {
// Module: crate::sample
// Provides: {"impl_1402"}
// Dependencies: {}
impl Strategy for SelectorStrategy { type Tree = SelectorValueTree ; type Value = Selector ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (SelectorValueTree { rng : runner . new_rng () , reverse_bias_increment : num :: u64 :: BinarySearch :: new (u64 :: MAX) , }) } }
};
}
