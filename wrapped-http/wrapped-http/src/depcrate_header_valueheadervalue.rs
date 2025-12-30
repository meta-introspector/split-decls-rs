// Generated macro for HeaderValue (struct)
macro_rules! Depcrate_header_valueHeaderValue {
() => {
// Module: crate::header::value
// Provides: {"HeaderValue"}
// Dependencies: {}
# [doc = " Represents an HTTP header field value."] # [doc = ""] # [doc = " In practice, HTTP header field values are usually valid ASCII. However, the"] # [doc = " HTTP spec allows for a header value to contain opaque bytes as well. In this"] # [doc = " case, the header field value is not able to be represented as a string."] # [doc = ""] # [doc = " To handle this, the `HeaderValue` is usable as a type and can be compared"] # [doc = " with strings and implements `Debug`. A `to_str` fn is provided that returns"] # [doc = " an `Err` if the header value contains non visible ascii characters."] # [derive (Clone)] pub struct HeaderValue { inner : Bytes , is_sensitive : bool , }
};
}
