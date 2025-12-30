// Generated macro for impl_74 (impl)
macro_rules! Depcrate_util_http_dateimpl_74 {
() => {
// Module: crate::util::http_date
// Provides: {"impl_74"}
// Dependencies: {}
impl HttpDate { pub (crate) fn from_val (val : & HeaderValue) -> Option < Self > { val . to_str () . ok () ? . parse () . ok () } }
};
}
