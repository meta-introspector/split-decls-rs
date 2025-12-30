// Generated macro for ipv6_netmask (function)
macro_rules! Depcrate_x509_commonipv6_netmask {
() => {
// Module: crate::x509::common
// Provides: {"ipv6_netmask"}
// Dependencies: {}
fn ipv6_netmask (num : u128) -> Result < u32 , CryptographyError > { if num . leading_ones () + num . trailing_zeros () != 128 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid netmask") ,)) ; } Ok ((! num) . leading_zeros ()) }
};
}
