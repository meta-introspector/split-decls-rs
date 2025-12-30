// Generated macro for impl_66 (impl)
macro_rules! Depcrate_dns_stringimpl_66 {
() => {
// Module: crate::dns_string
// Provides: {"impl_66"}
// Dependencies: {}
impl Display for DnsString { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , core :: fmt :: Error > { write ! (f , "{}" , escape_ascii (self . 0 . as_slice ())) } }
};
}
