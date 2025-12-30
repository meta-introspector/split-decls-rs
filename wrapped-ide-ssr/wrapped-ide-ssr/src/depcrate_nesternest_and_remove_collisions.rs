// Generated macro for nest_and_remove_collisions (function)
macro_rules! Depcrate_nesternest_and_remove_collisions {
() => {
// Module: crate::nester
// Provides: {"nest_and_remove_collisions"}
// Dependencies: {}
pub (crate) fn nest_and_remove_collisions (mut matches : Vec < Match > , sema : & hir :: Semantics < '_ , ide_db :: RootDatabase > ,) -> SsrMatches { matches . sort_by (| a , b | a . depth . cmp (& b . depth) . then_with (| | a . rule_index . cmp (& b . rule_index))) ; let mut collector = MatchCollector :: default () ; for m in matches { collector . add_match (m , sema) ; } collector . into () }
};
}
