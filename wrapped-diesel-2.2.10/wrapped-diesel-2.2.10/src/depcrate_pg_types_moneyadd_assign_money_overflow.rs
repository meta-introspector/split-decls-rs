// Generated macro for add_assign_money_overflow (function)
macro_rules! Depcrate_pg_types_moneyadd_assign_money_overflow {
() => {
// Module: crate::pg::types::money
// Provides: {"add_assign_money_overflow"}
// Dependencies: {}
# [test] # [should_panic (expected = "overflow adding money amounts")] fn add_assign_money_overflow () { let mut c1 = PgMoney (i64 :: MAX) ; c1 += PgMoney (1) ; }
};
}
