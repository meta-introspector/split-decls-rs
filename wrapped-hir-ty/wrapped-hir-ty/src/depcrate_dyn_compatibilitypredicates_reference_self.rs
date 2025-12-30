// Generated macro for predicates_reference_self (function)
macro_rules! Depcrate_dyn_compatibilitypredicates_reference_self {
() => {
// Module: crate::dyn_compatibility
// Provides: {"predicates_reference_self"}
// Dependencies: {}
fn predicates_reference_self (db : & dyn HirDatabase , trait_ : TraitId) -> bool { GenericPredicates :: query_explicit (db , trait_ . into ()) . iter_identity_copied () . any (| pred | predicate_references_self (db , trait_ , pred , AllowSelfProjection :: No)) }
};
}
