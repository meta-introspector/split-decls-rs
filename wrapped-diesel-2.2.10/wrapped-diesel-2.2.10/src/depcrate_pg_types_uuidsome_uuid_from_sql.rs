// Generated macro for some_uuid_from_sql (function)
macro_rules! Depcrate_pg_types_uuidsome_uuid_from_sql {
() => {
// Module: crate::pg::types::uuid
// Provides: {"some_uuid_from_sql"}
// Dependencies: {}
# [test] fn some_uuid_from_sql () { let bytes = [0xFF_u8 , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0x61 , 0x62 , 0x63 , 0x64 , 0x65 , 0x66 , 0x31 , 0x32 ,] ; let input_uuid = uuid :: Uuid :: from_slice (& bytes) . unwrap () ; let output_uuid = < uuid :: Uuid as FromSql < Uuid , Pg > > :: from_sql (PgValue :: for_test (input_uuid . as_bytes ())) . unwrap () ; assert_eq ! (input_uuid , output_uuid) ; }
};
}
