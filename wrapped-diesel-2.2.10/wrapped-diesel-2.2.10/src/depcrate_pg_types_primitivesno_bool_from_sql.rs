// Generated macro for no_bool_from_sql (function)
macro_rules! Depcrate_pg_types_primitivesno_bool_from_sql {
() => {
// Module: crate::pg::types::primitives
// Provides: {"no_bool_from_sql"}
// Dependencies: {}
# [test] fn no_bool_from_sql () { let result = < bool as FromSql < sql_types :: Bool , Pg > > :: from_nullable_sql (None) ; assert_eq ! (result . unwrap_err () . to_string () , "Unexpected null for non-null column") ; }
};
}
