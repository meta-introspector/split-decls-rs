// Generated macro for create_ip_network (function)
macro_rules! Depcrate_x509_commoncreate_ip_network {
() => {
// Module: crate::x509::common
// Provides: {"create_ip_network"}
// Dependencies: {}
fn create_ip_network < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let prefix = match data . len () { 8 => { let num = u32 :: from_be_bytes (data [4 ..] . try_into () . unwrap ()) ; ipv4_netmask (num) } 32 => { let num = u128 :: from_be_bytes (data [16 ..] . try_into () . unwrap ()) ; ipv6_netmask (num) } _ => Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("Invalid IPNetwork, must be 8 bytes for IPv4 and 32 bytes for IPv6. Found length: {}" , data . len ()) ,))) , } ; let base = types :: IPADDRESS_IPADDRESS . get (py) ? . call1 ((pyo3 :: types :: PyBytes :: new (py , & data [.. data . len () / 2]) ,)) ? ; let net = format ! ("{}/{}" , base . getattr (pyo3 :: intern ! (py , "exploded")) ? . extract ::< pyo3 :: pybacked :: PyBackedStr > () ?, prefix ?) ; let addr = types :: IPADDRESS_IPNETWORK . get (py) ? . call1 ((net ,)) ? ; Ok (types :: IP_ADDRESS . get (py) ? . call1 ((addr ,)) ?) }
};
}
