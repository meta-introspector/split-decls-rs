// Generated macro for Chain (struct)
macro_rules! DepcrateChain {
() => {
// Module: crate
// Provides: {"Chain"}
// Dependencies: {}
# [doc = " Iterator of a chain of source errors."] # [doc = ""] # [doc = " This type is the iterator returned by [`Report::chain`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use eyre::Report;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " pub fn underlying_io_error_kind(error: &Report) -> Option<io::ErrorKind> {"] # [doc = "     for cause in error.chain() {"] # [doc = "         if let Some(io_error) = cause.downcast_ref::<io::Error>() {"] # [doc = "             return Some(io_error.kind());"] # [doc = "         }"] # [doc = "     }"] # [doc = "     None"] # [doc = " }"] # [doc = " ```"] # [derive (Clone)] # [allow (missing_debug_implementations)] pub struct Chain < 'a > { state : crate :: chain :: ChainState < 'a > , }
};
}
