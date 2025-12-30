// Generated macro for impl_31 (impl)
macro_rules! Depcrate_ipextimpl_31 {
() => {
// Module: crate::ipext
// Provides: {"impl_31"}
// Dependencies: {}
impl IpStep for Ipv6Addr { fn replace_one (& mut self) -> Self { mem :: replace (self , Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1)) } fn replace_zero (& mut self) -> Self { mem :: replace (self , Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 0)) } fn add_one (& self) -> Self { self . saturating_add (1) } fn sub_one (& self) -> Self { self . saturating_sub (1) } }
};
}
