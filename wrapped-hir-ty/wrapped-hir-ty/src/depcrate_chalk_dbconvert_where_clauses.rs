// Generated macro for convert_where_clauses (function)
macro_rules! Depcrate_chalk_dbconvert_where_clauses {
() => {
// Module: crate::chalk_db
// Provides: {"convert_where_clauses"}
// Dependencies: {}
# [doc = " Returns instantiated predicates."] pub (super) fn convert_where_clauses (db : & dyn HirDatabase , def : GenericDefId , substs : & Substitution ,) -> Vec < chalk_ir :: QuantifiedWhereClause < Interner > > { db . generic_predicates (def) . iter () . cloned () . map (| pred | pred . substitute (Interner , substs)) . collect () }
};
}
