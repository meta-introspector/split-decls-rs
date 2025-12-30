// Generated macro for impl_1604 (impl)
macro_rules! Depcrate_x509impl_1604 {
() => {
// Module: crate::x509
// Provides: {"impl_1604"}
// Dependencies: {}
impl fmt :: Debug for GeneralNameRef { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (email) = self . email () { formatter . write_str (email) } else if let Some (dnsname) = self . dnsname () { formatter . write_str (dnsname) } else if let Some (uri) = self . uri () { formatter . write_str (uri) } else if let Some (ipaddress) = self . ipaddress () { let address = < [u8 ; 16] > :: try_from (ipaddress) . map (IpAddr :: from) . or_else (| _ | < [u8 ; 4] > :: try_from (ipaddress) . map (IpAddr :: from)) ; match address { Ok (a) => fmt :: Debug :: fmt (& a , formatter) , Err (_) => fmt :: Debug :: fmt (ipaddress , formatter) , } } else { formatter . write_str ("(empty)") } } }
};
}
