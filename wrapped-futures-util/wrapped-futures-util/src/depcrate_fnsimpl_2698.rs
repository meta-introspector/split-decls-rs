// Generated macro for impl_2698 (impl)
macro_rules! Depcrate_fnsimpl_2698 {
() => {
// Module: crate::fns
// Provides: {"impl_2698"}
// Dependencies: {}
impl < T , A , R > FnOnce1 < A > for T where T : FnOnce (A) -> R , { type Output = R ; fn call_once (self , arg : A) -> R { self (arg) } }
};
}
