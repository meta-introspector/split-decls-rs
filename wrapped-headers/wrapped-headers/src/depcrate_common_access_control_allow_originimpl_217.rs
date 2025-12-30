// Generated macro for impl_217 (impl)
macro_rules! Depcrate_common_access_control_allow_originimpl_217 {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"impl_217"}
// Dependencies: {}
impl AccessControlAllowOrigin { # [doc = " `Access-Control-Allow-Origin: *`"] pub const ANY : AccessControlAllowOrigin = AccessControlAllowOrigin (OriginOrAny :: Any) ; # [doc = " `Access-Control-Allow-Origin: null`"] pub const NULL : AccessControlAllowOrigin = AccessControlAllowOrigin (OriginOrAny :: Origin (Origin :: NULL)) ; # [doc = " Returns the origin if there's one specified."] pub fn origin (& self) -> Option < & Origin > { match self . 0 { OriginOrAny :: Origin (ref origin) => Some (origin) , _ => None , } } }
};
}
