// Generated macro for Pfx (struct)
macro_rules! Depcrate_pkcs12Pfx {
() => {
// Module: crate::pkcs12
// Provides: {"Pfx"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct Pfx < 'a > { pub version : u8 , pub auth_safe : pkcs7 :: ContentInfo < 'a > , pub mac_data : Option < MacData < 'a > > , }
};
}
