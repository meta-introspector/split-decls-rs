// Generated macro for impl_555 (impl)
macro_rules! Depcrate_uriimpl_555 {
() => {
// Module: crate::uri
// Provides: {"impl_555"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for Uri { type Error = InvalidUri ; # [inline] fn try_from (vec : Vec < u8 >) -> Result < Self , Self :: Error > { Uri :: from_shared (Bytes :: from (vec)) } }
};
}
