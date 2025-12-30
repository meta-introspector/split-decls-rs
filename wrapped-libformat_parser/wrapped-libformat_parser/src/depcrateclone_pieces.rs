// Generated macro for clone_pieces (function)
macro_rules! Depcrateclone_pieces {
() => {
// Module: crate
// Provides: {"clone_pieces"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn clone_pieces (FormatArgsHandle (piece_slice , s) : & FormatArgsHandle ,) -> FormatArgsHandle { let PieceSlice { base_ptr , len , cap } = * piece_slice ; let v = unsafe { Vec :: from_raw_parts (base_ptr , len , cap) } ; let cloned_v = v . clone () ; v . leak () ; let piece_slice = PieceSlice { len : cloned_v . len () , cap : cloned_v . capacity () , base_ptr : cloned_v . leak () . as_mut_ptr () , } ; let RustString { ptr , len , cap } = * s ; let s = unsafe { String :: from_raw_parts (ptr as * mut u8 , len , cap) } ; let cloned_s = s . clone () ; s . leak () ; let rust_string = RustString { len : cloned_s . len () , cap : cloned_s . capacity () , ptr : cloned_s . leak () . as_ptr () , } ; FormatArgsHandle (piece_slice , rust_string) }
};
}
