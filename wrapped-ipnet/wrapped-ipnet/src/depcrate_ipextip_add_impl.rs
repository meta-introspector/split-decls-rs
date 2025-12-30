// Generated macro for ip_add_impl (macro)
macro_rules! Depcrate_ipextip_add_impl {
() => {
// Module: crate::ipext
// Provides: {"ip_add_impl"}
// Dependencies: {}
macro_rules ! ip_add_impl { ($ lhs : ty , $ rhs : ty , $ output : ty , $ inner : ty) => (impl IpAdd <$ rhs > for $ lhs { type Output = $ output ; fn saturating_add (self , rhs : $ rhs) -> $ output { let lhs : $ inner = self . into () ; let rhs : $ inner = rhs . into () ; (lhs . saturating_add (rhs . into ())) . into () } }) }
};
}
