// Generated macro for impl_79 (impl)
macro_rules! Depcrate_ipnetimpl_79 {
() => {
// Module: crate::ipnet
// Provides: {"impl_79"}
// Dependencies: {}
impl fmt :: Display for Ipv4Net { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "{}/{}" , self . addr , self . prefix_len) } }
};
}
