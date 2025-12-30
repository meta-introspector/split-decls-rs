// Generated macro for v6_masked_address_from_sql (function)
macro_rules! Depcrate_pg_types_ipnet_addressv6_masked_address_from_sql {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"v6_masked_address_from_sql"}
// Dependencies: {}
# [test] fn v6_masked_address_from_sql () { macro_rules ! test_to_sql { ($ ty : ty , $ net_type : expr , $ last_byte : expr) => { let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let test_address = IpNet :: V6 (Ipv6Net :: new (Ipv6Addr :: new (0xfd , 0 , 0 , 0 , 0 , 0 , 0 , 1) , 64) . unwrap ()) ; ToSql ::<$ ty , Pg >:: to_sql (& test_address , & mut bytes) . unwrap () ; } assert_eq ! (buffer , vec ! [PGSQL_AF_INET6 , 64 , $ net_type , 16 , 0 , 0xfd , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , $ last_byte ,]) ; } ; } test_to_sql ! (Inet , 0 , 1) ; test_to_sql ! (Cidr , 1 , 0) ; }
};
}
