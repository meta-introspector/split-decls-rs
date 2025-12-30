// Generated macro for impl_3028 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3028 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3028"}
// Dependencies: {}
impl SubAssign for PgMoney { # [doc = " # Panics"] # [doc = ""] # [doc = " Performs a checked subtraction, and will `panic!` on underflow in both `debug` and `release`."] fn sub_assign (& mut self , rhs : PgMoney) { self . 0 = self . 0 . checked_sub (rhs . 0) . expect ("underflow subtracting money amounts") } }
};
}
