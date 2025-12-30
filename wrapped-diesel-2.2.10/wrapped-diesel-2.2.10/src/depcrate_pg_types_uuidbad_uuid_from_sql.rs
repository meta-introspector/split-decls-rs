// Generated macro for bad_uuid_from_sql (function)
macro_rules! Depcrate_pg_types_uuidbad_uuid_from_sql {
() => {
// Module: crate::pg::types::uuid
// Provides: {"bad_uuid_from_sql"}
// Dependencies: {}
# [test] fn bad_uuid_from_sql () { let uuid = uuid :: Uuid :: from_sql (PgValue :: for_test (b"boom")) ; assert ! (uuid . is_err ()) ; let error_message = uuid . unwrap_err () . to_string () ; assert ! (error_message . starts_with ("invalid")) ; assert ! (error_message . contains ("length")) ; assert ! (error_message . contains ("expected 16")) ; assert ! (error_message . ends_with ("found 4")) ; }
};
}
