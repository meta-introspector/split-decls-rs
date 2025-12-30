// Generated macro for some_v4address_from_sql (function)
macro_rules! Depcrate_pg_types_ipnet_addresssome_v4address_from_sql {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"some_v4address_from_sql"}
// Dependencies: {}
# [test] fn some_v4address_from_sql () { macro_rules ! test_some_address_from_sql { ($ ty : tt) => { let input_address = IpNet :: V4 (Ipv4Net :: new (Ipv4Addr :: new (127 , 0 , 0 , 1) , 32) . unwrap ()) ; let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql ::<$ ty , Pg >:: to_sql (& input_address , & mut bytes) . unwrap () ; } let output_address = FromSql ::<$ ty , Pg >:: from_sql (PgValue :: for_test (& buffer)) . unwrap () ; assert_eq ! (input_address , output_address) ; } ; } test_some_address_from_sql ! (Cidr) ; test_some_address_from_sql ! (Inet) ; }
};
}
