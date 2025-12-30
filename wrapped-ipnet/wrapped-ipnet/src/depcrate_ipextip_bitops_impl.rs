// Generated macro for ip_bitops_impl (macro)
macro_rules! Depcrate_ipextip_bitops_impl {
() => {
// Module: crate::ipext
// Provides: {"ip_bitops_impl"}
// Dependencies: {}
macro_rules ! ip_bitops_impl { ($ (($ lhs : ty , $ rhs : ty , $ t : ty) ,) *) => { $ (impl IpBitAnd <$ rhs > for $ lhs { type Output = $ lhs ; fn bitand (self , rhs : $ rhs) -> $ lhs { let lhs : $ t = self . into () ; let rhs : $ t = rhs . into () ; (lhs & rhs) . into () } } impl IpBitOr <$ rhs > for $ lhs { type Output = $ lhs ; fn bitor (self , rhs : $ rhs) -> $ lhs { let lhs : $ t = self . into () ; let rhs : $ t = rhs . into () ; (lhs | rhs) . into () } }) * } }
};
}
