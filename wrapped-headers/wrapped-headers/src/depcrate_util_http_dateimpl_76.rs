// Generated macro for impl_76 (impl)
macro_rules! Depcrate_util_http_dateimpl_76 {
() => {
// Module: crate::util::http_date
// Provides: {"impl_76"}
// Dependencies: {}
impl super :: TryFromValues for HttpDate { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , crate :: Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . and_then (HttpDate :: from_val) . ok_or_else (crate :: Error :: invalid) } }
};
}
