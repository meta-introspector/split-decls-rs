// Generated macro for of1024_impl (function)
macro_rules! Depcrate_compressorof1024_impl {
() => {
// Module: crate::compressor
// Provides: {"of1024_impl"}
// Dependencies: {}
# [inline (always)] unsafe fn of1024_impl (cv : & mut X8) { let p = transpose_inv (* cv ^ rounds_p (* cv)) ; cv . 4 = p . 4 ; cv . 5 = p . 5 ; cv . 6 = p . 6 ; cv . 7 = p . 7 ; }
};
}
