// Generated macro for LoopSource (enum)
macro_rules! Depcrate_hirLoopSource {
() => {
// Module: crate::hir
// Provides: {"LoopSource"}
// Dependencies: {}
# [doc = " The loop type that yielded an `ExprKind::Loop`."] # [derive (Copy , Clone , PartialEq , Debug , HashStable_Generic)] pub enum LoopSource { # [doc = " A `loop { .. }` loop."] Loop , # [doc = " A `while _ { .. }` loop."] While , # [doc = " A `for _ in _ { .. }` loop."] ForLoop , }
};
}
