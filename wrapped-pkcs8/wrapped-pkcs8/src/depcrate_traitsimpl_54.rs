// Generated macro for impl_54 (impl)
macro_rules! Depcrate_traitsimpl_54 {
() => {
// Module: crate::traits
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > DecodePrivateKey for T where T : for < 'a > TryFrom < PrivateKeyInfoRef < 'a > , Error = Error > , { fn from_pkcs8_der (bytes : & [u8]) -> Result < Self > { Self :: try_from (PrivateKeyInfoRef :: try_from (bytes) ?) } }
};
}
