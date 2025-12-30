// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < F : FloatRepr > FuzzOpEvalOutputs < F > { fn all_match (self) -> bool { [self . cxx_apf , self . hard] . into_iter () . flatten () . all (| x | x == self . rs_apf) } }
};
}
