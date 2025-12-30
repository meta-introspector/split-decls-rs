// Generated macro for MacData (struct)
macro_rules! Depcrate_pkcs12MacData {
() => {
// Module: crate::pkcs12
// Provides: {"MacData"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct MacData < 'a > { pub mac : pkcs7 :: DigestInfo < 'a > , pub salt : & 'a [u8] , # [default (1u64)] pub iterations : u64 , }
};
}
