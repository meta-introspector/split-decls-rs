// Generated macro for CachedDate (struct)
macro_rules! Depcrate_common_dateCachedDate {
() => {
// Module: crate::common::date
// Provides: {"CachedDate"}
// Dependencies: {}
struct CachedDate { bytes : [u8 ; DATE_VALUE_LENGTH] , pos : usize , # [cfg (feature = "http2")] header_value : HeaderValue , next_update : SystemTime , }
};
}
