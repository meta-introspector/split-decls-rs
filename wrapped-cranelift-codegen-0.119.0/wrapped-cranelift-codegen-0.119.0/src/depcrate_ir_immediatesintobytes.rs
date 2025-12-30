// Generated macro for IntoBytes (trait)
macro_rules! Depcrate_ir_immediatesIntoBytes {
() => {
// Module: crate::ir::immediates
// Provides: {"IntoBytes"}
// Dependencies: {}
# [doc = " Convert a type into a vector of bytes; all implementors in this file must use little-endian"] # [doc = " orderings of bytes to match WebAssembly's little-endianness."] pub trait IntoBytes { # [doc = " Return the little-endian byte representation of the implementing type."] fn into_bytes (self) -> Vec < u8 > ; }
};
}
