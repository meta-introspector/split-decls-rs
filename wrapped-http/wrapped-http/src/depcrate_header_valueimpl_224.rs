// Generated macro for impl_224 (impl)
macro_rules! Depcrate_header_valueimpl_224 {
() => {
// Module: crate::header::value
// Provides: {"impl_224"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for HeaderValue { type Error = InvalidHeaderValue ; # [inline] fn try_from (vec : Vec < u8 >) -> Result < Self , Self :: Error > { HeaderValue :: from_shared (vec . into ()) } }
};
}
