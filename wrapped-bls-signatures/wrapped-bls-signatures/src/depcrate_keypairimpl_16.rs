// Generated macro for impl_16 (impl)
macro_rules! Depcrate_keypairimpl_16 {
() => {
// Module: crate::keypair
// Provides: {"impl_16"}
// Dependencies: {}
impl TryFrom < & [u8] > for Keypair { type Error = BlsError ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != BLS_KEYPAIR_SIZE { return Err (BlsError :: ParseFromBytes) ; } Ok (Self { secret : SecretKey :: try_from (& bytes [.. BLS_SECRET_KEY_SIZE]) ? , public : Pubkey (bytes [BLS_SECRET_KEY_SIZE ..] . try_into () . map_err (| _ | BlsError :: ParseFromBytes) ? ,) , }) } }
};
}
