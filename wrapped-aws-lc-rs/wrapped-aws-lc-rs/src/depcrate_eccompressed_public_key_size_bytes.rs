// Generated macro for compressed_public_key_size_bytes (function)
macro_rules! Depcrate_eccompressed_public_key_size_bytes {
() => {
// Module: crate::ec
// Provides: {"compressed_public_key_size_bytes"}
// Dependencies: {}
# [inline] pub (crate) const fn compressed_public_key_size_bytes (curve_field_bits : usize) -> usize { 1 + (curve_field_bits + 7) / 8 }
};
}
