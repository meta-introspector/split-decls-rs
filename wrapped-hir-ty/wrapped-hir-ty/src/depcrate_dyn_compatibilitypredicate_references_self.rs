// Generated macro for predicate_references_self (function)
macro_rules! Depcrate_dyn_compatibilitypredicate_references_self {
() => {
// Module: crate::dyn_compatibility
// Provides: {"predicate_references_self"}
// Dependencies: {}
fn predicate_references_self < 'db > (db : & 'db dyn HirDatabase , trait_ : TraitId , predicate : Clause < 'db > , allow_self_projection : AllowSelfProjection ,) -> bool { match predicate . kind () . skip_binder () { ClauseKind :: Trait (trait_pred) => trait_pred . trait_ref . args . iter () . skip (1) . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , allow_self_projection) }) , ClauseKind :: Projection (proj_pred) => { proj_pred . projection_term . args . iter () . skip (1) . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , allow_self_projection) }) } _ => false , } }
};
}
