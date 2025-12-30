// Generated macro for sub_assign_money_underflow (function)
macro_rules! Depcrate_pg_types_moneysub_assign_money_underflow {
() => {
// Module: crate::pg::types::money
// Provides: {"sub_assign_money_underflow"}
// Dependencies: {}
# [test] # [should_panic (expected = "underflow subtracting money amounts")] fn sub_assign_money_underflow () { let mut c1 = PgMoney (i64 :: MIN) ; c1 -= PgMoney (1) ; }
};
}
