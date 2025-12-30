// Generated macro for v4_masked_address_to_sql (function)
macro_rules! Depcrate_pg_types_ipnet_addressv4_masked_address_to_sql {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"v4_masked_address_to_sql"}
// Dependencies: {}
# [test] fn v4_masked_address_to_sql () { macro_rules ! test_to_sql { ($ ty : ty , $ net_type : expr , $ last_byte : expr) => { let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let test_address = IpNet :: V4 (Ipv4Net :: new (Ipv4Addr :: new (192 , 168 , 0 , 1) , 24) . unwrap ()) ; ToSql ::<$ ty , Pg >:: to_sql (& test_address , & mut bytes) . unwrap () ; } assert_eq ! (buffer , vec ! [PGSQL_AF_INET , 24 , $ net_type , 4 , 192 , 168 , 0 , $ last_byte]) ; } ; } test_to_sql ! (Inet , 0 , 1) ; test_to_sql ! (Cidr , 1 , 0) ; }
};
}
