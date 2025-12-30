// Generated macro for ip_sub_impl (macro)
macro_rules! Depcrate_ipextip_sub_impl {
() => {
// Module: crate::ipext
// Provides: {"ip_sub_impl"}
// Dependencies: {}
macro_rules ! ip_sub_impl { ($ lhs : ty , $ rhs : ty , $ output : ty , $ inner : ty) => (impl IpSub <$ rhs > for $ lhs { type Output = $ output ; fn saturating_sub (self , rhs : $ rhs) -> $ output { let lhs : $ inner = self . into () ; let rhs : $ inner = rhs . into () ; (lhs . saturating_sub (rhs . into ())) . into () } }) }
};
}
