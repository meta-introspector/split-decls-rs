// Generated macro for decompress_error (function)
macro_rules! Depcrate_inflatedecompress_error {
() => {
// Module: crate::inflate
// Provides: {"decompress_error"}
// Dependencies: {}
# [cfg (feature = "with-alloc")] fn decompress_error (status : TINFLStatus , output : Vec < u8 >) -> Result < Vec < u8 > , DecompressError > { Err (DecompressError { status , output }) }
};
}
