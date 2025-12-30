// Generated macro for uuid_to_sql (function)
macro_rules! Depcrate_pg_types_uuiduuid_to_sql {
() => {
// Module: crate::pg::types::uuid
// Provides: {"uuid_to_sql"}
// Dependencies: {}
# [test] fn uuid_to_sql () { use crate :: query_builder :: bind_collector :: ByteWrapper ; let mut buffer = Vec :: new () ; let bytes = [0xFF_u8 , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0xFF , 0x61 , 0x62 , 0x63 , 0x64 , 0x65 , 0x66 , 0x31 , 0x32 ,] ; let test_uuid = uuid :: Uuid :: from_slice (& bytes) . unwrap () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < Uuid , Pg > :: to_sql (& test_uuid , & mut bytes) . unwrap () ; assert_eq ! (& buffer , test_uuid . as_bytes ()) ; }
};
}
