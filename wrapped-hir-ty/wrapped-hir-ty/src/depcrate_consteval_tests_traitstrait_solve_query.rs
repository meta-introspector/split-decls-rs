// Generated macro for trait_solve_query (function)
macro_rules! Depcrate_consteval_tests_traitstrait_solve_query {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"trait_solve_query"}
// Dependencies: {}
# [doc = " Solve a trait goal using Chalk."] pub (crate) fn trait_solve_query (db : & dyn HirDatabase , krate : Crate , block : Option < BlockId > , goal : Canonical < InEnvironment < Goal > > ,) -> Option < Solution > { let _p = tracing :: info_span ! ("trait_solve_query" , detail = ? match & goal . value . goal . data (Interner) { GoalData :: DomainGoal (DomainGoal :: Holds (WhereClause :: Implemented (it))) => db . trait_signature (it . hir_trait_id ()) . name . display (db , Edition :: LATEST) . to_string () , GoalData :: DomainGoal (DomainGoal :: Holds (WhereClause :: AliasEq (_))) => "alias_eq" . to_owned () , _ => "??" . to_owned () , }) . entered () ; if let GoalData :: DomainGoal (DomainGoal :: Holds (WhereClause :: AliasEq (AliasEq { alias : AliasTy :: Projection (projection_ty) , .. }))) = & goal . value . goal . data (Interner) && let TyKind :: BoundVar (_) = projection_ty . self_type_parameter (db) . kind (Interner) { return Some (Solution :: Ambig (Guidance :: Unknown)) ; } let goal = goal . try_fold_with (& mut UnevaluatedConstEvaluatorFolder { db } , DebruijnIndex :: INNERMOST) . unwrap () ; let u_canonical = chalk_ir :: UCanonical { canonical : goal , universes : 1 } ; solve (db , krate , block , & u_canonical) }
};
}
