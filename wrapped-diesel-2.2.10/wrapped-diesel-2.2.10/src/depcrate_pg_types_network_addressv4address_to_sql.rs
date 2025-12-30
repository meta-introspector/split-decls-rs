// Generated macro for v4address_to_sql (function)
macro_rules! Depcrate_pg_types_network_addressv4address_to_sql {
() => {
// Module: crate::pg::types::network_address
// Provides: {"v4address_to_sql"}
// Dependencies: {}
# [test] fn v4address_to_sql () { macro_rules ! test_to_sql { ($ ty : ty , $ net_type : expr) => { let mut buffer = Vec :: new () ; { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; let test_address = IpNetwork :: V4 (Ipv4Network :: new (Ipv4Addr :: new (127 , 0 , 0 , 1) , 32) . unwrap ()) ; ToSql ::<$ ty , Pg >:: to_sql (& test_address , & mut bytes) . unwrap () ; } assert_eq ! (buffer , vec ! [PGSQL_AF_INET , 32 , $ net_type , 4 , 127 , 0 , 0 , 1]) ; } ; } test_to_sql ! (Inet , 0) ; test_to_sql ! (Cidr , 1) ; }
};
}
