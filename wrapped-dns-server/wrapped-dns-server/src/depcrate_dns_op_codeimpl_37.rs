// Generated macro for impl_37 (impl)
macro_rules! Depcrate_dns_op_codeimpl_37 {
() => {
// Module: crate::dns_op_code
// Provides: {"impl_37"}
// Dependencies: {}
impl DnsOpCode { # [must_use] pub fn new (value : u8) -> Self { match value { 0 => DnsOpCode :: Query , 1 => DnsOpCode :: InverseQuery , 2 => DnsOpCode :: Status , other => DnsOpCode :: Reserved (other) , } } # [must_use] pub fn num (& self) -> u8 { match self { DnsOpCode :: Query => 0 , DnsOpCode :: InverseQuery => 1 , DnsOpCode :: Status => 2 , DnsOpCode :: Reserved (other) => * other , } } }
};
}
