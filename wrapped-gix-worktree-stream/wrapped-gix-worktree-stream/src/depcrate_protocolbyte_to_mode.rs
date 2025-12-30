// Generated macro for byte_to_mode (function)
macro_rules! Depcrate_protocolbyte_to_mode {
() => {
// Module: crate::protocol
// Provides: {"byte_to_mode"}
// Dependencies: {}
fn byte_to_mode (b : u8) -> gix_object :: tree :: EntryMode { use gix_object :: tree :: EntryKind :: * ; match b { 0 => Tree , 1 => Blob , 2 => BlobExecutable , 3 => Link , 4 => Commit , _ => unreachable ! ("BUG: we control the protocol") , } . into () }
};
}
