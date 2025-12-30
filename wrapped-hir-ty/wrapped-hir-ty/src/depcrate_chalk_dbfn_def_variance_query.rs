// Generated macro for fn_def_variance_query (function)
macro_rules! Depcrate_chalk_dbfn_def_variance_query {
() => {
// Module: crate::chalk_db
// Provides: {"fn_def_variance_query"}
// Dependencies: {}
pub (crate) fn fn_def_variance_query (db : & dyn HirDatabase , callable_def : CallableDefId ,) -> Variances { Variances :: from_iter (Interner , db . variances_of (GenericDefId :: from_callable (db , callable_def)) . as_deref () . unwrap_or_default () . iter () . map (| v | match v { crate :: variance :: Variance :: Covariant => chalk_ir :: Variance :: Covariant , crate :: variance :: Variance :: Invariant => chalk_ir :: Variance :: Invariant , crate :: variance :: Variance :: Contravariant => chalk_ir :: Variance :: Contravariant , crate :: variance :: Variance :: Bivariant => chalk_ir :: Variance :: Invariant , }) ,) }
};
}
