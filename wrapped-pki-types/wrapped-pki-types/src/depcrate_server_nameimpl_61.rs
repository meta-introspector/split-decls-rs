// Generated macro for impl_61 (impl)
macro_rules! Depcrate_server_nameimpl_61 {
() => {
// Module: crate::server_name
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for ServerName < 'a > { type Error = InvalidDnsNameError ; fn try_from (value : & 'a [u8]) -> Result < Self , Self :: Error > { match str :: from_utf8 (value) { Ok (s) => Self :: try_from (s) , Err (_) => Err (InvalidDnsNameError) , } } }
};
}
