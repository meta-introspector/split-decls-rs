// Generated macro for impl_datum_query (function)
macro_rules! Depcrate_chalk_dbimpl_datum_query {
() => {
// Module: crate::chalk_db
// Provides: {"impl_datum_query"}
// Dependencies: {}
pub (crate) fn impl_datum_query (db : & dyn HirDatabase , krate : Crate , impl_id : ImplId ,) -> Arc < ImplDatum > { let _p = tracing :: info_span ! ("impl_datum_query") . entered () ; debug ! ("impl_datum {:?}" , impl_id) ; let impl_ : hir_def :: ImplId = from_chalk (db , impl_id) ; impl_def_datum (db , krate , impl_) }
};
}
