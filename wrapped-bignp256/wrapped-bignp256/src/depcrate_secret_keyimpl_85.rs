// Generated macro for impl_85 (impl)
macro_rules! Depcrate_secret_keyimpl_85 {
() => {
// Module: crate::secret_key
// Provides: {"impl_85"}
// Dependencies: {}
impl TryFrom < pkcs8 :: PrivateKeyInfoRef < '_ > > for SecretKey { type Error = pkcs8 :: Error ; fn try_from (private_key_info : pkcs8 :: PrivateKeyInfoRef < '_ >) -> pkcs8 :: Result < Self > { private_key_info . algorithm . assert_oids (ALGORITHM_OID , BignP256 :: OID) ? ; Self :: from_slice (private_key_info . private_key . as_bytes ()) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) } }
};
}
