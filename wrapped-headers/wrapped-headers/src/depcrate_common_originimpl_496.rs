// Generated macro for impl_496 (impl)
macro_rules! Depcrate_common_originimpl_496 {
() => {
// Module: crate::common::origin
// Provides: {"impl_496"}
// Dependencies: {}
impl < 'a > From < & 'a OriginOrNull > for HeaderValue { fn from (origin : & 'a OriginOrNull) -> HeaderValue { match origin { OriginOrNull :: Origin (ref scheme , ref auth) => { let s = format ! ("{}://{}" , scheme , auth) ; let bytes = Bytes :: from (s) ; HeaderValue :: from_maybe_shared (bytes) . expect ("Scheme and Authority are valid header values") } OriginOrNull :: Null => HeaderValue :: from_static ("null") , } } }
};
}
