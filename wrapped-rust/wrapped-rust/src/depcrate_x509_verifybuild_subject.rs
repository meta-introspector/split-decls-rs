// Generated macro for build_subject (function)
macro_rules! Depcrate_x509_verifybuild_subject {
() => {
// Module: crate::x509::verify
// Provides: {"build_subject"}
// Dependencies: {}
fn build_subject < 'a > (py : pyo3 :: Python < '_ > , subject : & 'a SubjectOwner ,) -> pyo3 :: PyResult < Subject < 'a > > { match subject { SubjectOwner :: DNSName (dns_name) => { let dns_name = DNSName :: new (dns_name) . ok_or_else (| | pyo3 :: exceptions :: PyValueError :: new_err ("invalid domain name")) ? ; Ok (Subject :: DNS (dns_name)) } SubjectOwner :: IPAddress (ip_addr) => { let ip_addr = IPAddress :: from_bytes (ip_addr . as_bytes (py)) . ok_or_else (| | pyo3 :: exceptions :: PyValueError :: new_err ("invalid IP address")) ? ; Ok (Subject :: IP (ip_addr)) } } }
};
}
