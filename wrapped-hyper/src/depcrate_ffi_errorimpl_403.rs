// Generated macro for impl_403 (impl)
macro_rules! Depcrate_ffi_errorimpl_403 {
() => {
// Module: crate::ffi::error
// Provides: {"impl_403"}
// Dependencies: {}
impl hyper_error { fn code (& self) -> hyper_code { use crate :: error :: Kind as ErrorKind ; use crate :: error :: User ; match self . 0 . kind () { ErrorKind :: Parse (_) => hyper_code :: HYPERE_INVALID_PEER_MESSAGE , ErrorKind :: IncompleteMessage => hyper_code :: HYPERE_UNEXPECTED_EOF , ErrorKind :: User (User :: AbortedByCallback) => hyper_code :: HYPERE_ABORTED_BY_CALLBACK , _ => hyper_code :: HYPERE_ERROR , } } fn print_to (& self , dst : & mut [u8]) -> usize { use std :: io :: Write ; let mut dst = std :: io :: Cursor :: new (dst) ; let _ = write ! (dst , "{}" , & self . 0) ; dst . position () as usize } }
};
}
