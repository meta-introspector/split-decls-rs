// Generated macro for bad_address_from_sql (function)
macro_rules! Depcrate_pg_types_ipnet_addressbad_address_from_sql {
() => {
// Module: crate::pg::types::ipnet_address
// Provides: {"bad_address_from_sql"}
// Dependencies: {}
# [test] fn bad_address_from_sql () { macro_rules ! bad_address_from_sql { ($ ty : tt) => { let address : Result < IpNet , _ > = FromSql ::<$ ty , Pg >:: from_sql (PgValue :: for_test (& [7 , PGSQL_AF_INET , 0])) ; assert_eq ! (address . unwrap_err () . to_string () , "invalid network address format. input is too short.") ; } ; } bad_address_from_sql ! (Inet) ; bad_address_from_sql ! (Cidr) ; }
};
}
