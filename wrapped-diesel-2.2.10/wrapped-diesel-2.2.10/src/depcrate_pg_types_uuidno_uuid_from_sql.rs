// Generated macro for no_uuid_from_sql (function)
macro_rules! Depcrate_pg_types_uuidno_uuid_from_sql {
() => {
// Module: crate::pg::types::uuid
// Provides: {"no_uuid_from_sql"}
// Dependencies: {}
# [test] fn no_uuid_from_sql () { let uuid = uuid :: Uuid :: from_nullable_sql (None) ; assert_eq ! (uuid . unwrap_err () . to_string () , "Unexpected null for non-null column") ; }
};
}
