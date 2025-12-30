// Generated macro for sub_money_underflow (function)
macro_rules! Depcrate_pg_types_moneysub_money_underflow {
() => {
// Module: crate::pg::types::money
// Provides: {"sub_money_underflow"}
// Dependencies: {}
# [test] # [should_panic (expected = "underflow subtracting money amounts")] fn sub_money_underflow () { let c1 = PgMoney (i64 :: MIN) ; let c2 = PgMoney (1) ; let _underflow = c1 - c2 ; }
};
}
