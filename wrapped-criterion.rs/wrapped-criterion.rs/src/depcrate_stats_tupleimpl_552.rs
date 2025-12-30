// Generated macro for impl_552 (impl)
macro_rules! Depcrate_stats_tupleimpl_552 {
() => {
// Module: crate::stats::tuple
// Provides: {"impl_552"}
// Dependencies: {}
impl < A , B , C , D > Tuple for (A , B , C , D) where A : Copy , B : Copy , C : Copy , D : Copy , { type Distributions = (Distribution < A > , Distribution < B > , Distribution < C > , Distribution < D > ,) ; type Builder = (Vec < A > , Vec < B > , Vec < C > , Vec < D >) ; }
};
}
