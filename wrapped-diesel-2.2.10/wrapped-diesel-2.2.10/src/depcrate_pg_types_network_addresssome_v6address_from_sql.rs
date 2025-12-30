// Generated macro for some_v6address_from_sql (function)
macro_rules! Depcrate_pg_types_network_addresssome_v6address_from_sql {
() => {
// Module: crate::pg::types::network_address
// Provides: {"some_v6address_from_sql"}
// Dependencies: {}
# [test] fn some_v6address_from_sql () { macro_rules ! test_some_address_from_sql { ($ ty : tt) => { let input_address = IpNetwork :: V6 (Ipv6Network :: new (Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1) , 64) . unwrap ()) ; let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql ::<$ ty , Pg >:: to_sql (& input_address , & mut bytes) . unwrap () ; } let output_address = FromSql ::<$ ty , Pg >:: from_sql (PgValue :: for_test (& buffer)) . unwrap () ; assert_eq ! (input_address , output_address) ; } ; } test_some_address_from_sql ! (Inet) ; test_some_address_from_sql ! (Cidr) ; }
};
}
