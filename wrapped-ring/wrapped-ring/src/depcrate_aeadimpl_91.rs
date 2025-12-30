// Generated macro for impl_91 (impl)
macro_rules! Depcrate_aeadimpl_91 {
() => {
// Module: crate::aead
// Provides: {"impl_91"}
// Dependencies: {}
impl TryFrom < & [u8] > for Tag { type Error = error :: Unspecified ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { let raw_tag : [u8 ; TAG_LEN] = value . try_into () . map_err (| _ | error :: Unspecified) ? ; Ok (Self :: from (raw_tag)) } }
};
}
