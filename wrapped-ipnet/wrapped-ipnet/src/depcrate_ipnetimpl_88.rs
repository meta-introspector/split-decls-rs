// Generated macro for impl_88 (impl)
macro_rules! Depcrate_ipnetimpl_88 {
() => {
// Module: crate::ipnet
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a > Contains < & 'a IpAddr > for IpNet { fn contains (& self , other : & IpAddr) -> bool { match (* self , * other) { (IpNet :: V4 (ref a) , IpAddr :: V4 (ref b)) => a . contains (b) , (IpNet :: V6 (ref a) , IpAddr :: V6 (ref b)) => a . contains (b) , _ => false , } } }
};
}
