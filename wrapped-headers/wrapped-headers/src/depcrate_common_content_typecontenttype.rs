// Generated macro for ContentType (struct)
macro_rules! Depcrate_common_content_typeContentType {
() => {
// Module: crate::common::content_type
// Provides: {"ContentType"}
// Dependencies: {}
# [doc = " `Content-Type` header, defined in"] # [doc = " [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-3.1.1.5)"] # [doc = ""] # [doc = " The `Content-Type` header field indicates the media type of the"] # [doc = " associated representation: either the representation enclosed in the"] # [doc = " message payload or the selected representation, as determined by the"] # [doc = " message semantics.  The indicated media type defines both the data"] # [doc = " format and how that data is intended to be processed by a recipient,"] # [doc = " within the scope of the received message semantics, after any content"] # [doc = " codings indicated by Content-Encoding are decoded."] # [doc = ""] # [doc = " Although the `mime` crate allows the mime options to be any slice, this crate"] # [doc = " forces the use of Vec. This is to make sure the same header can't have more than 1 type. If"] # [doc = " this is an issue, it's possible to implement `Header` on a custom struct."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Content-Type = media-type"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `text/html; charset=utf-8`"] # [doc = " * `application/json`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ContentType;"] # [doc = ""] # [doc = " let ct = ContentType::json();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct ContentType (Mime) ;
};
}
