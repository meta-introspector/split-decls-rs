// Generated macro for add_money_overflow (function)
macro_rules! Depcrate_pg_types_moneyadd_money_overflow {
() => {
// Module: crate::pg::types::money
// Provides: {"add_money_overflow"}
// Dependencies: {}
# [test] # [should_panic (expected = "overflow adding money amounts")] fn add_money_overflow () { let c1 = PgMoney (i64 :: MAX) ; let c2 = PgMoney (1) ; let _overflow = c1 + c2 ; }
};
}
