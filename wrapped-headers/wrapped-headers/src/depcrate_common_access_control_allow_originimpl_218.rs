// Generated macro for impl_218 (impl)
macro_rules! Depcrate_common_access_control_allow_originimpl_218 {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"impl_218"}
// Dependencies: {}
impl TryFrom < & str > for AccessControlAllowOrigin { type Error = Error ; fn try_from (s : & str) -> Result < Self , Error > { let header_value = HeaderValue :: from_str (s) . map_err (| _ | Error :: invalid ()) ? ; let origin = OriginOrAny :: try_from (& header_value) ? ; Ok (Self (origin)) } }
};
}
