// Generated macro for impl_404 (impl)
macro_rules! Depcrate_uri_authorityimpl_404 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_404"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for Authority { type Error = InvalidUri ; # [inline] fn try_from (vec : Vec < u8 >) -> Result < Self , Self :: Error > { Authority :: from_shared (vec . into ()) } }
};
}
