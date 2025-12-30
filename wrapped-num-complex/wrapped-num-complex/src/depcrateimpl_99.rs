// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : Clone + Num > Complex < T > { # [doc = " Find the gaussian integer corresponding to the true ratio rounded towards zero."] fn div_trunc (& self , divisor : & Self) -> Self { let Complex { re , im } = self / divisor ; Complex :: new (re . clone () - re % T :: one () , im . clone () - im % T :: one ()) } }
};
}
