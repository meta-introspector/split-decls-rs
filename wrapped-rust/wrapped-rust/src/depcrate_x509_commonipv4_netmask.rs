// Generated macro for ipv4_netmask (function)
macro_rules! Depcrate_x509_commonipv4_netmask {
() => {
// Module: crate::x509::common
// Provides: {"ipv4_netmask"}
// Dependencies: {}
fn ipv4_netmask (num : u32) -> Result < u32 , CryptographyError > { if num . leading_ones () + num . trailing_zeros () != 32 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid netmask") ,)) ; } Ok ((! num) . leading_zeros ()) }
};
}
