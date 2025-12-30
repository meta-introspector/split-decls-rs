// Generated macro for impl_78 (impl)
macro_rules! Depcrate_util_http_dateimpl_78 {
() => {
// Module: crate::util::http_date
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a > From < & 'a HttpDate > for HeaderValue { fn from (date : & 'a HttpDate) -> HeaderValue { let s = date . to_string () ; let bytes = Bytes :: from (s) ; HeaderValue :: from_maybe_shared (bytes) . expect ("HttpDate always is a valid value") } }
};
}
