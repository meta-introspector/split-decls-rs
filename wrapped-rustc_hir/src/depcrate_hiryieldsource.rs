// Generated macro for YieldSource (enum)
macro_rules! Depcrate_hirYieldSource {
() => {
// Module: crate::hir
// Provides: {"YieldSource"}
// Dependencies: {}
# [doc = " The yield kind that caused an `ExprKind::Yield`."] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum YieldSource { # [doc = " An `<expr>.await`."] Await { expr : Option < HirId > } , # [doc = " A plain `yield`."] Yield , }
};
}
