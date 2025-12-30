// Generated macro for impl_3026 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3026 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3026"}
// Dependencies: {}
impl AddAssign for PgMoney { # [doc = " # Panics"] # [doc = ""] # [doc = " Performs a checked addition, and will `panic!` on overflow in both `debug` and `release`."] fn add_assign (& mut self , rhs : PgMoney) { self . 0 = self . 0 . checked_add (rhs . 0) . expect ("overflow adding money amounts") } }
};
}
