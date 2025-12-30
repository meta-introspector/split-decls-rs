// Generated macro for impl_219 (impl)
macro_rules! Depcrate_common_access_control_allow_originimpl_219 {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"impl_219"}
// Dependencies: {}
impl TryFrom < & HeaderValue > for OriginOrAny { type Error = Error ; fn try_from (header_value : & HeaderValue) -> Result < Self , Error > { Origin :: try_from_value (header_value) . map (OriginOrAny :: Origin) . ok_or_else (Error :: invalid) } }
};
}
