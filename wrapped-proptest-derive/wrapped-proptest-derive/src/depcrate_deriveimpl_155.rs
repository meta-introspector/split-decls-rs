// Generated macro for impl_155 (impl)
macro_rules! Depcrate_deriveimpl_155 {
() => {
// Module: crate::derive
// Provides: {"impl_155"}
// Dependencies: {}
impl StratAcc < Ctor > { # [doc = " Finishes off the accumulator by returning"] # [doc = " a `.prop_map(<closure>)` of the strategies."] fn finish (self , closure : MapClosure) -> StratPair { pair_map (self . consume () , closure) } }
};
}
