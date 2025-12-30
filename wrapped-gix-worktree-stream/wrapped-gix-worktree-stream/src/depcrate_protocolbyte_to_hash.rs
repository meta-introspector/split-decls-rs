// Generated macro for byte_to_hash (function)
macro_rules! Depcrate_protocolbyte_to_hash {
() => {
// Module: crate::protocol
// Provides: {"byte_to_hash"}
// Dependencies: {}
fn byte_to_hash (b : u8) -> gix_hash :: Kind { match b { 0 => gix_hash :: Kind :: Sha1 , _ => unreachable ! ("BUG: we control the protocol") , } }
};
}
