// Generated macro for mode_to_byte (function)
macro_rules! Depcrate_protocolmode_to_byte {
() => {
// Module: crate::protocol
// Provides: {"mode_to_byte"}
// Dependencies: {}
fn mode_to_byte (m : gix_object :: tree :: EntryMode) -> u8 { use gix_object :: tree :: EntryKind :: * ; match m . kind () { Tree => 0 , Blob => 1 , BlobExecutable => 2 , Link => 3 , Commit => 4 , } }
};
}
