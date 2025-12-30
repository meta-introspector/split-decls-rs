// Generated macro for collect_pieces (function)
macro_rules! Depcratecollect_pieces {
() => {
// Module: crate
// Provides: {"collect_pieces"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn collect_pieces (input : * const libc :: c_char , append_newline : bool , parse_mode : crate :: ffi :: ParseMode ,) -> FormatArgsHandle { let str = unsafe { CStr :: from_ptr (input) } ; let str = str . to_str () . unwrap () . to_owned () ; let s = & str ; let s = unsafe { std :: mem :: transmute :: < & '_ str , & 'static str > (s) } ; let pieces : Vec < ffi :: Piece < '_ > > = rust :: collect_pieces (s , None , None , append_newline , parse_mode) . into_iter () . map (Into :: into) . collect () ; let piece_slice = PieceSlice { len : pieces . len () , cap : pieces . capacity () , base_ptr : pieces . leak () . as_mut_ptr () , } ; let rust_string = RustString { len : str . len () , cap : str . capacity () , ptr : str . leak () . as_ptr () , } ; FormatArgsHandle (piece_slice , rust_string) }
};
}
