// Generated macro for cchar_from_sql (function)
macro_rules! Depcrate_pg_types_primitivescchar_from_sql {
() => {
// Module: crate::pg::types::primitives
// Provides: {"cchar_from_sql"}
// Dependencies: {}
# [test] fn cchar_from_sql () { let result = < u8 as FromSql < sql_types :: CChar , Pg > > :: from_nullable_sql (None) ; assert_eq ! (result . unwrap_err () . to_string () , "Unexpected null for non-null column") ; }
};
}
