// Generated macro for update_and_header_value (function)
macro_rules! Depcrate_common_dateupdate_and_header_value {
() => {
// Module: crate::common::date
// Provides: {"update_and_header_value"}
// Dependencies: {}
# [cfg (feature = "http2")] pub (crate) fn update_and_header_value () -> HeaderValue { CACHED . with (| cache | { let mut cache = cache . borrow_mut () ; cache . check () ; cache . header_value . clone () }) }
};
}
