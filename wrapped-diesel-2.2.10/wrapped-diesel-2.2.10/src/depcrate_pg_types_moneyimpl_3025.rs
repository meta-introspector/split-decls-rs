// Generated macro for impl_3025 (impl)
macro_rules! Depcrate_pg_types_moneyimpl_3025 {
() => {
// Module: crate::pg::types::money
// Provides: {"impl_3025"}
// Dependencies: {}
impl Add for PgMoney { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " Performs a checked addition, and will `panic!` on overflow in both `debug` and `release`."] fn add (self , rhs : PgMoney) -> Self :: Output { self . 0 . checked_add (rhs . 0) . map (PgMoney) . expect ("overflow adding money amounts") } }
};
}
