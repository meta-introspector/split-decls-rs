// Generated macro for impl_221 (impl)
macro_rules! Depcrate_common_access_control_allow_originimpl_221 {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > From < & 'a OriginOrAny > for HeaderValue { fn from (origin : & 'a OriginOrAny) -> HeaderValue { match origin { OriginOrAny :: Origin (ref origin) => origin . to_value () , OriginOrAny :: Any => HeaderValue :: from_static ("*") , } } }
};
}
