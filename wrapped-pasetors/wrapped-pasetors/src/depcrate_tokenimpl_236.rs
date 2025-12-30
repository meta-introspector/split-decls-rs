// Generated macro for impl_236 (impl)
macro_rules! Depcrate_tokenimpl_236 {
() => {
// Module: crate::token
// Provides: {"impl_236"}
// Dependencies: {}
impl < T : Purpose < V > , V : Version > TryFrom < & String > for UntrustedToken < T , V > { type Error = Error ; # [doc = " This fails if `value` is not a PASETO token or it has invalid base64 encoding."] fn try_from (value : & String) -> Result < Self , Self :: Error > { Self :: try_from (value . as_str ()) } }
};
}
