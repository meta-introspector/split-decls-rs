// Generated macro for adt_variance_query (function)
macro_rules! Depcrate_chalk_dbadt_variance_query {
() => {
// Module: crate::chalk_db
// Provides: {"adt_variance_query"}
// Dependencies: {}
pub (crate) fn adt_variance_query (db : & dyn HirDatabase , adt_id : hir_def :: AdtId) -> Variances { Variances :: from_iter (Interner , db . variances_of (adt_id . into ()) . as_deref () . unwrap_or_default () . iter () . map (| v | match v { crate :: variance :: Variance :: Covariant => chalk_ir :: Variance :: Covariant , crate :: variance :: Variance :: Invariant => chalk_ir :: Variance :: Invariant , crate :: variance :: Variance :: Contravariant => chalk_ir :: Variance :: Contravariant , crate :: variance :: Variance :: Bivariant => chalk_ir :: Variance :: Invariant , }) ,) }
};
}
