// Generated macro for impl_17 (impl)
macro_rules! Depcrate_keypairimpl_17 {
() => {
// Module: crate::keypair
// Provides: {"impl_17"}
// Dependencies: {}
impl From < & Keypair > for [u8 ; BLS_KEYPAIR_SIZE] { fn from (keypair : & Keypair) -> Self { let mut bytes = [0u8 ; BLS_KEYPAIR_SIZE] ; bytes [.. BLS_SECRET_KEY_SIZE] . copy_from_slice (& Into :: < [u8 ; BLS_SECRET_KEY_SIZE] > :: into (& keypair . secret)) ; bytes [BLS_SECRET_KEY_SIZE ..] . copy_from_slice (& keypair . public . 0) ; bytes } }
};
}
