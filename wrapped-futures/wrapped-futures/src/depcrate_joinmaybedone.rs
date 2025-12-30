// Generated macro for MaybeDone (enum)
macro_rules! Depcrate_joinMaybeDone {
() => {
// Module: crate::join
// Provides: {"MaybeDone"}
// Dependencies: {}
enum MaybeDone < A : Future > { NotYet (Collapsed < A >) , Done (A :: Item) , Gone , }
};
}
