// Generated macro for BitMapBackendError (enum)
macro_rules! Depcrate_errorBitMapBackendError {
() => {
// Module: crate::error
// Provides: {"BitMapBackendError"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Indicates some error occurs within the bitmap backend"] pub enum BitMapBackendError { # [doc = " The buffer provided is invalid, for example, wrong pixel buffer size"] InvalidBuffer , # [doc = " Some IO error occurs while the bitmap manipulation"] IOError (std :: io :: Error) , # [cfg (all (feature = "gif" , not (target_arch = "wasm32") , feature = "image"))] GifEncodingError (gif :: EncodingError) , # [cfg (all (not (target_arch = "wasm32") , feature = "image"))] # [doc = " Image encoding error"] ImageError (ImageError) , }
};
}
