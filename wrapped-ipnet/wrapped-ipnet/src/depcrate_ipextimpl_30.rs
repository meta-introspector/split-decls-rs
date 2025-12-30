// Generated macro for impl_30 (impl)
macro_rules! Depcrate_ipextimpl_30 {
() => {
// Module: crate::ipext
// Provides: {"impl_30"}
// Dependencies: {}
impl IpStep for Ipv4Addr { fn replace_one (& mut self) -> Self { mem :: replace (self , Ipv4Addr :: new (0 , 0 , 0 , 1)) } fn replace_zero (& mut self) -> Self { mem :: replace (self , Ipv4Addr :: new (0 , 0 , 0 , 0)) } fn add_one (& self) -> Self { self . saturating_add (1) } fn sub_one (& self) -> Self { self . saturating_sub (1) } }
};
}
