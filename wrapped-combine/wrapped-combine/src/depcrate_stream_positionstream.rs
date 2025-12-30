// Generated macro for Stream (struct)
macro_rules! Depcrate_stream_positionStream {
() => {
// Module: crate::stream::position
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " The `Stream<Input>` struct maintains the current position in the stream `Input` using"] # [doc = " the `Positioner` trait to track the position."] # [doc = ""] # [doc = " ```"] # [doc = " # #![cfg(feature = \"std\")]"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::stream::easy;"] # [doc = " # use combine::stream::position;"] # [doc = " # fn main() {"] # [doc = "     let result = token(b'9')"] # [doc = "         .message(\"Not a nine\")"] # [doc = "         .easy_parse(position::Stream::new(&b\"8\"[..]));"] # [doc = "     assert_eq!(result, Err(easy::Errors {"] # [doc = "         position: 0,"] # [doc = "         errors: vec!["] # [doc = "             easy::Error::Unexpected(b'8'.into()),"] # [doc = "             easy::Error::Expected(b'9'.into()),"] # [doc = "             easy::Error::Message(\"Not a nine\".into())"] # [doc = "         ]"] # [doc = "     }));"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct Stream < Input , X > { # [doc = " The input stream used when items are requested"] pub input : Input , # [doc = " The positioner used to update the current position"] pub positioner : X , }
};
}
