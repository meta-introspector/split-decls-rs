// Generated macro for impl_93 (impl)
macro_rules! Depcrate_pubkey_conversionimpl_93 {
() => {
// Module: crate::pubkey::conversion
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl TryFrom < & [u8] > for PubkeyProjective { type Error = BlsError ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != BLS_PUBLIC_KEY_AFFINE_SIZE { return Err (BlsError :: ParseFromBytes) ; } let public_affine = Pubkey (bytes . try_into () . unwrap ()) ; public_affine . try_into () } }
};
}
