// Generated macro for impl_433 (impl)
macro_rules! Depcrate_uri_pathimpl_433 {
() => {
// Module: crate::uri::path
// Provides: {"impl_433"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for PathAndQuery { type Error = InvalidUri ; # [inline] fn try_from (vec : Vec < u8 >) -> Result < Self , Self :: Error > { PathAndQuery :: from_shared (vec . into ()) } }
};
}
