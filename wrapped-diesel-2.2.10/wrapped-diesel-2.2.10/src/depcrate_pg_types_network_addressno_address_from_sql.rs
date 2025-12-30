// Generated macro for no_address_from_sql (function)
macro_rules! Depcrate_pg_types_network_addressno_address_from_sql {
() => {
// Module: crate::pg::types::network_address
// Provides: {"no_address_from_sql"}
// Dependencies: {}
# [test] fn no_address_from_sql () { macro_rules ! test_no_address_from_sql { ($ ty : ty) => { let address : Result < IpNetwork , _ > = FromSql ::<$ ty , Pg >:: from_nullable_sql (None) ; assert_eq ! (address . unwrap_err () . to_string () , "Unexpected null for non-null column") ; } ; } test_no_address_from_sql ! (Inet) ; test_no_address_from_sql ! (Cidr) ; }
};
}
