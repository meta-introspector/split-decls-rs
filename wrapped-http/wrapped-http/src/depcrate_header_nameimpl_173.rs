// Generated macro for impl_173 (impl)
macro_rules! Depcrate_header_nameimpl_173 {
() => {
// Module: crate::header::name
// Provides: {"impl_173"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for HeaderName { type Error = InvalidHeaderName ; # [inline] fn try_from (vec : Vec < u8 >) -> Result < Self , Self :: Error > { Self :: from_bytes (& vec) } }
};
}
