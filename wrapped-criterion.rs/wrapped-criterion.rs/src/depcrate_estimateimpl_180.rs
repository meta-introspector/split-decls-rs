// Generated macro for impl_180 (impl)
macro_rules! Depcrate_estimateimpl_180 {
() => {
// Module: crate::estimate
// Provides: {"impl_180"}
// Dependencies: {}
impl ChangeDistributions { pub fn get (& self , stat : Statistic) -> & Distribution < f64 > { match stat { Statistic :: Mean => & self . mean , Statistic :: Median => & self . median , _ => panic ! ("Unexpected statistic") , } } }
};
}
