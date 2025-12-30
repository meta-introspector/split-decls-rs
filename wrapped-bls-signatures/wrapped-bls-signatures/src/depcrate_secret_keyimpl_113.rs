// Generated macro for impl_113 (impl)
macro_rules! Depcrate_secret_keyimpl_113 {
() => {
// Module: crate::secret_key
// Provides: {"impl_113"}
// Dependencies: {}
impl From < & SecretKey > for [u8 ; BLS_SECRET_KEY_SIZE] { fn from (secret_key : & SecretKey) -> Self { secret_key . 0 . to_bytes_le () } }
};
}
