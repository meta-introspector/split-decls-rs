// Generated macro for elaborate_clause_supertraits (function)
macro_rules! Depcrate_utilselaborate_clause_supertraits {
() => {
// Module: crate::utils
// Provides: {"elaborate_clause_supertraits"}
// Dependencies: {}
pub (super) fn elaborate_clause_supertraits (db : & dyn HirDatabase , clauses : impl Iterator < Item = WhereClause > ,) -> ClauseElaborator < '_ > { let mut elaborator = ClauseElaborator { db , stack : Vec :: new () , seen : FxHashSet :: default () } ; elaborator . extend_deduped (clauses) ; elaborator }
};
}
