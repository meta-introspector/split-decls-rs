// Generated macro for impl_495 (impl)
macro_rules! Depcrate_common_originimpl_495 {
() => {
// Module: crate::common::origin
// Provides: {"impl_495"}
// Dependencies: {}
impl TryFromValues for OriginOrNull { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . and_then (OriginOrNull :: try_from_value) . ok_or_else (Error :: invalid) } }
};
}
