// Generated macro for impl_1385 (impl)
macro_rules! Depcrate_sampleimpl_1385 {
() => {
// Module: crate::sample
// Provides: {"impl_1385"}
// Dependencies: {}
impl < T : fmt :: Debug + Clone + 'static > Strategy for Subsequence < T > { type Tree = SubsequenceValueTree < T > ; type Value = Vec < T > ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (SubsequenceValueTree { values : Arc :: clone (& self . values) , inner : self . bit_strategy . new_tree (runner) ? , }) } }
};
}
