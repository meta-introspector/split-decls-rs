// Generated macro for impl_230 (impl)
macro_rules! Depcrate_stream_readimpl_230 {
() => {
// Module: crate::stream::read
// Provides: {"impl_230"}
// Dependencies: {}
impl < R > Stream < R > where R : Read , { # [doc = " Creates a `StreamOnce` instance from a value implementing `std::io::Read`."] # [doc = ""] # [doc = " NOTE: This type do not implement `Positioned` and `Clone` and must be wrapped with types"] # [doc = "     such as `BufferedStreamRef` and `State` to become a `Stream` which can be parsed"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![cfg(feature = \"std\")]"] # [doc = " # extern crate combine;"] # [doc = " use combine::*;"] # [doc = " use combine::parser::byte::*;"] # [doc = " use combine::stream::read;"] # [doc = " use combine::stream::buffered;"] # [doc = " use combine::stream::position;"] # [doc = " use std::io::Read;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " let input: &[u8] = b\"123,\";"] # [doc = " let stream = buffered::Stream::new(position::Stream::new(read::Stream::new(input)), 1);"] # [doc = " let result = (many(digit()), byte(b','))"] # [doc = "     .parse(stream)"] # [doc = "     .map(|t| t.0);"] # [doc = " assert_eq!(result, Ok((vec![b'1', b'2', b'3'], b',')));"] # [doc = " # }"] # [doc = " ```"] pub fn new (read : R) -> Stream < R > { Stream { bytes : read . bytes () , } } }
};
}
