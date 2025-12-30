// Generated macro for StreamErrorFor (type)
macro_rules! Depcrate_streamStreamErrorFor {
() => {
// Module: crate::stream
// Provides: {"StreamErrorFor"}
// Dependencies: {}
# [doc = " Convenience alias over the `StreamError` for the input stream `Input`"] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use]"] # [doc = " extern crate combine;"] # [doc = " use combine::{easy, Parser, Stream, many1};"] # [doc = " use combine::parser::char::letter;"] # [doc = " use combine::stream::StreamErrorFor;"] # [doc = " use combine::error::{ParseError, StreamError};"] # [doc = ""] # [doc = " parser!{"] # [doc = "    fn parser[Input]()(Input) -> String"] # [doc = "     where [ Input: Stream<Token = char>, ]"] # [doc = "     {"] # [doc = "         many1(letter()).and_then(|word: String| {"] # [doc = "             if word == \"combine\" {"] # [doc = "                 Ok(word)"] # [doc = "             } else {"] # [doc = "                 // The alias makes it easy to refer to the `StreamError` type of `Input`"] # [doc = "                 Err(StreamErrorFor::<Input>::expected_static_message(\"combine\"))"] # [doc = "             }"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = " }"] # [doc = " ```"] pub type StreamErrorFor < Input > = < < Input as StreamOnce > :: Error as ParseError < < Input as StreamOnce > :: Token , < Input as StreamOnce > :: Range , < Input as StreamOnce > :: Position , > > :: StreamError ;
};
}
