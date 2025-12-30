// Generated macro for verify_retry_integrity (function)
macro_rules! Depcrate_packetverify_retry_integrity {
() => {
// Module: crate::packet
// Provides: {"verify_retry_integrity"}
// Dependencies: {}
pub fn verify_retry_integrity (b : & octets :: OctetsMut , odcid : & [u8] , version : u32 ,) -> Result < () > { const TAG_LEN : usize = RETRY_AEAD_ALG . tag_len () ; let tag = compute_retry_integrity_tag (b , odcid , version) ? ; crypto :: verify_slices_are_equal (& b . as_ref () [.. TAG_LEN] , tag . as_ref ()) }
};
}
