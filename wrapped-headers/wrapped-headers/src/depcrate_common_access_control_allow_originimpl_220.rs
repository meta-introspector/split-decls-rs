// Generated macro for impl_220 (impl)
macro_rules! Depcrate_common_access_control_allow_originimpl_220 {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"impl_220"}
// Dependencies: {}
impl TryFromValues for OriginOrAny { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . and_then (| value | { if value == "*" { return Some (OriginOrAny :: Any) ; } Origin :: try_from_value (value) . map (OriginOrAny :: Origin) }) . ok_or_else (Error :: invalid) } }
};
}
