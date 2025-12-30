// Generated macro for write_to (function)
macro_rules! Depcrate_extension_sparsewrite_to {
() => {
// Module: crate::extension::sparse
// Provides: {"write_to"}
// Dependencies: {}
# [doc = " Serialize the sparse index extension to `out`"] pub fn write_to (mut out : impl std :: io :: Write) -> Result < () , std :: io :: Error > { out . write_all (& SIGNATURE) ? ; out . write_all (& 0_u32 . to_be_bytes ()) ? ; Ok (()) }
};
}
