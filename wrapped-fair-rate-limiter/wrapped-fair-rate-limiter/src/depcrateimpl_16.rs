// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Display for IpAddrKey { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { if let Some (addr) = to_ipv4_mapped (& self . 0) { write ! (f , "{addr}") } else { write ! (f , "{}" , self . 0) } } }
};
}
