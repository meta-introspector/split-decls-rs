// Generated macro for impl_87 (impl)
macro_rules! Depcrate_ipnetimpl_87 {
() => {
// Module: crate::ipnet
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a > Contains < & 'a IpNet > for IpNet { fn contains (& self , other : & IpNet) -> bool { match (* self , * other) { (IpNet :: V4 (ref a) , IpNet :: V4 (ref b)) => a . contains (b) , (IpNet :: V6 (ref a) , IpNet :: V6 (ref b)) => a . contains (b) , _ => false , } } }
};
}
