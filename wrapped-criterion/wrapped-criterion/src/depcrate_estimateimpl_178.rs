// Generated macro for impl_178 (impl)
macro_rules! Depcrate_estimateimpl_178 {
() => {
// Module: crate::estimate
// Provides: {"impl_178"}
// Dependencies: {}
impl ChangeEstimates { pub fn get (& self , stat : Statistic) -> & Estimate { match stat { Statistic :: Mean => & self . mean , Statistic :: Median => & self . median , _ => panic ! ("Unexpected statistic") , } } }
};
}
