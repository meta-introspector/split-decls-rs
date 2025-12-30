// Generated macro for impl_112 (impl)
macro_rules! Depcrate_secret_keyimpl_112 {
() => {
// Module: crate::secret_key
// Provides: {"impl_112"}
// Dependencies: {}
impl TryFrom < & [u8] > for SecretKey { type Error = BlsError ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != BLS_SECRET_KEY_SIZE { return Err (BlsError :: ParseFromBytes) ; } let scalar : Option < Scalar > = Scalar :: from_bytes_le (bytes . try_into () . unwrap ()) . into () ; scalar . ok_or (BlsError :: FieldDecode) . map (Self) } }
};
}
