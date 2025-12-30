// Generated macro for impl_549 (impl)
macro_rules! Depcrate_stats_tupleimpl_549 {
() => {
// Module: crate::stats::tuple
// Provides: {"impl_549"}
// Dependencies: {}
impl < A , B , C > Tuple for (A , B , C) where A : Copy , B : Copy , C : Copy , { type Distributions = (Distribution < A > , Distribution < B > , Distribution < C >) ; type Builder = (Vec < A > , Vec < B > , Vec < C >) ; }
};
}
