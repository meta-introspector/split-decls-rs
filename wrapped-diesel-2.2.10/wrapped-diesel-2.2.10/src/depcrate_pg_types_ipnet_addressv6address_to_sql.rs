// Generated macro for v6address_to_sql (function)
macro_rules! Depcrate_pg_types_ipnet_addressv6address_to_sql {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"v6address_to_sql"}
// Dependencies: {}
# [test] fn v6address_to_sql () { macro_rules ! test_to_sql { ($ ty : ty , $ net_type : expr) => { let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let test_address = IpNet :: V6 (Ipv6Net :: new (Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1) , 64) . unwrap ()) ; ToSql ::<$ ty , Pg >:: to_sql (& test_address , & mut bytes) . unwrap () ; } assert_eq ! (buffer , vec ! [PGSQL_AF_INET6 , 64 , $ net_type , 16 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 ,]) ; } ; } test_to_sql ! (Inet , 0) ; test_to_sql ! (Cidr , 1) ; }
};
}
