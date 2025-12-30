// Generated macro for impl_112 (impl)
macro_rules! Depcrate_util_value_stringimpl_112 {
() => {
// Module: crate::util::value_string
// Provides: {"impl_112"}
// Dependencies: {}
impl super :: TryFromValues for HeaderValueString { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . map (HeaderValueString :: from_val) . unwrap_or_else (| | Err (Error :: invalid ())) } }
};
}
