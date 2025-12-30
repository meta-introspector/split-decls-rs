// Generated macro for sub_assign_money (function)
macro_rules! Depcrate_pg_types_moneysub_assign_money {
() => {
// Module: crate::pg::types::money
// Provides: {"sub_assign_money"}
// Dependencies: {}
# [test] fn sub_assign_money () { let mut c1 = PgMoney (123) ; c1 -= PgMoney (456) ; assert_eq ! (PgMoney (- 333) , c1) ; }
};
}
