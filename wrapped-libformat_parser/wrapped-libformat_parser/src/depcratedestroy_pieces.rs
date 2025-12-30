// Generated macro for destroy_pieces (function)
macro_rules! Depcratedestroy_pieces {
() => {
// Module: crate
// Provides: {"destroy_pieces"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn destroy_pieces (FormatArgsHandle (piece_slice , s) : FormatArgsHandle) { let PieceSlice { base_ptr , len , cap } = piece_slice ; drop (Vec :: from_raw_parts (base_ptr , len , cap)) ; let RustString { ptr , len , cap } = s ; drop (String :: from_raw_parts (ptr as * mut u8 , len , cap)) ; }
};
}
