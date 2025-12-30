// Generated macro for impl_79 (impl)
macro_rules! Depcrate_util_http_dateimpl_79 {
() => {
// Module: crate::util::http_date
// Provides: {"impl_79"}
// Dependencies: {}
impl FromStr for HttpDate { type Err = Error ; fn from_str (s : & str) -> Result < HttpDate , Error > { Ok (HttpDate (s . parse () . map_err (| _ | Error (())) ?)) } }
};
}
