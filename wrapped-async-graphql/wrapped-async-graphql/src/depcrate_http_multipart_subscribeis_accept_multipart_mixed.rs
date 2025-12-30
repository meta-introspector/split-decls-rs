// Generated macro for is_accept_multipart_mixed (function)
macro_rules! Depcrate_http_multipart_subscribeis_accept_multipart_mixed {
() => {
// Module: crate::http::multipart_subscribe
// Provides: {"is_accept_multipart_mixed"}
// Dependencies: {}
# [doc = " Check accept is multipart-mixed"] # [doc = ""] # [doc = " # Example header"] # [doc = ""] # [doc = " ```text"] # [doc = " Accept: multipart/mixed; boundary=\"graphql\"; subscriptionSpec=\"1.0\""] # [doc = " ```"] # [doc = ""] # [doc = " the value for boundary should always be `graphql`, and the value"] # [doc = " for `subscriptionSpec` should always be `1.0`."] # [doc = ""] # [doc = " Reference: <https://www.apollographql.com/docs/router/executing-operations/subscription-multipart-protocol/>"] pub fn is_accept_multipart_mixed (accept : & str) -> bool { for mime in parse_accept (accept) { if mime . type_ () == mime :: APPLICATION && mime . subtype () == mime :: JSON { return false ; } if mime . type_ () == mime :: MULTIPART && mime . subtype () == "mixed" && mime . get_param (mime :: BOUNDARY) . map (| value | value . as_str ()) == Some ("graphql") && mime . get_param ("subscriptionSpec") . map (| value | value . as_str ()) == Some ("1.0") { return true ; } } false }
};
}
