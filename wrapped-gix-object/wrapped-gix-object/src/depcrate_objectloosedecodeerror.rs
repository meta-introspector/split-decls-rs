// Generated macro for LooseDecodeError (enum)
macro_rules! Depcrate_objectLooseDecodeError {
() => {
// Module: crate::object
// Provides: {"LooseDecodeError"}
// Dependencies: {}
# [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum LooseDecodeError { # [error (transparent)] InvalidHeader (# [from] LooseHeaderDecodeError) , # [error (transparent)] InvalidContent (# [from] DecodeError) , # [error ("Object sized {size} does not fit into memory - this can happen on 32 bit systems")] OutOfMemory { size : u64 } , }
};
}
