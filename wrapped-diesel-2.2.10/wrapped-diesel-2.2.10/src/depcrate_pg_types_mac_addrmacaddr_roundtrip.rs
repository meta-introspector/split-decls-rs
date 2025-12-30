// Generated macro for macaddr_roundtrip (function)
macro_rules! Depcrate_pg_types_mac_addrmacaddr_roundtrip {
() => {
// Module: crate::pg::types::mac_addr
// Provides: {"macaddr_roundtrip"}
// Dependencies: {}
# [test] fn macaddr_roundtrip () { use crate :: query_builder :: bind_collector :: ByteWrapper ; let mut buffer = Vec :: new () ; let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let input_address = [0x52 , 0x54 , 0x00 , 0xfb , 0xc6 , 0x16] ; ToSql :: < MacAddr , Pg > :: to_sql (& input_address , & mut bytes) . unwrap () ; let output_address : [u8 ; 6] = FromSql :: from_sql (PgValue :: for_test (& buffer)) . unwrap () ; assert_eq ! (input_address , output_address) ; }
};
}
