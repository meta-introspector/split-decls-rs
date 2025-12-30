// Generated macro for impl_245 (impl)
macro_rules! Depcrate_rata_dieimpl_245 {
() => {
// Module: crate::rata_die
// Provides: {"impl_245"}
// Dependencies: {}
# [doc = " Shift a RataDie N days into the past"] impl Sub < i64 > for RataDie { type Output = Self ; fn sub (self , rhs : i64) -> Self :: Output { let result = Self (self . 0 - rhs) ; # [cfg (debug_assertions)] result . check () ; result } }
};
}
