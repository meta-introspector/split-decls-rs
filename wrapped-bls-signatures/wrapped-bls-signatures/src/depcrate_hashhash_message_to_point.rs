// Generated macro for hash_message_to_point (function)
macro_rules! Depcrate_hashhash_message_to_point {
() => {
// Module: crate::hash
// Provides: {"hash_message_to_point"}
// Dependencies: {}
# [doc = " Hash a message to a G2 point"] pub fn hash_message_to_point (message : & [u8]) -> G2Projective { G2Projective :: hash_to_curve (message , HASH_TO_POINT_DST , & []) }
};
}
