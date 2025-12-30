// Generated macro for impl_700 (impl)
macro_rules! Depcrate_algoimpl_700 {
() => {
// Module: crate::algo
// Provides: {"impl_700"}
// Dependencies: {}
impl < N , VM > Default for DfsSpace < N , VM > where VM : VisitMap < N > + Default , { fn default () -> Self { DfsSpace { dfs : Dfs { stack : < _ > :: default () , discovered : < _ > :: default () , } , } } }
};
}
