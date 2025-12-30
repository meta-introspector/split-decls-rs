// Generated macro for AlternativeExprs (enum)
macro_rules! Depcrate_term_searchAlternativeExprs {
() => {
// Module: crate::term_search
// Provides: {"AlternativeExprs"}
// Dependencies: {}
# [doc = " Helper enum to squash big number of alternative trees into `Many` variant as there is too many"] # [doc = " to take into account."] # [derive (Debug)] enum AlternativeExprs < 'db > { # [doc = " There are few trees, so we keep track of them all"] Few (FxHashSet < Expr < 'db > >) , # [doc = " There are too many trees to keep track of"] Many , }
};
}
