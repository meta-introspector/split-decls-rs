// Generated macro for impl_3027 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3027 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3027"}
// Dependencies: {}
impl Sub for PgMoney { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " Performs a checked subtraction, and will `panic!` on underflow in both `debug` and `release`."] fn sub (self , rhs : PgMoney) -> Self :: Output { self . 0 . checked_sub (rhs . 0) . map (PgMoney) . expect ("underflow subtracting money amounts") } }
};
}
