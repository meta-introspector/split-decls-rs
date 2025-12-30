// Generated macro for impl_1403 (impl)
macro_rules! Depcrate_sampleimpl_1403 {
() => {
// Module: crate::sample
// Provides: {"impl_1403"}
// Dependencies: {}
impl ValueTree for SelectorValueTree { type Value = Selector ; fn current (& self) -> Selector { Selector { rng : self . rng . clone () , bias_increment : u64 :: MAX - self . reverse_bias_increment . current () , } } fn simplify (& mut self) -> bool { self . reverse_bias_increment . simplify () } fn complicate (& mut self) -> bool { self . reverse_bias_increment . complicate () } }
};
}
