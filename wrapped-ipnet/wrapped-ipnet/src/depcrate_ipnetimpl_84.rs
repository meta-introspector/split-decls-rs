// Generated macro for impl_84 (impl)
macro_rules! Depcrate_ipnetimpl_84 {
() => {
// Module: crate::ipnet
// Provides: {"impl_84"}
// Dependencies: {}
impl fmt :: Display for Ipv6Net { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "{}/{}" , self . addr , self . prefix_len) } }
};
}
