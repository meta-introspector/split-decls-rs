// Generated macro for impl_72 (impl)
macro_rules! Depcrate_ipnetimpl_72 {
() => {
// Module: crate::ipnet
// Provides: {"impl_72"}
// Dependencies: {}
impl fmt :: Display for IpNet { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { IpNet :: V4 (ref a) => a . fmt (fmt) , IpNet :: V6 (ref a) => a . fmt (fmt) , } } }
};
}
