// Generated macro for impl_552 (impl)
macro_rules! Depcrate_uriimpl_552 {
() => {
// Module: crate::uri
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Uri { type Error = InvalidUri ; # [inline] fn try_from (t : & 'a str) -> Result < Self , Self :: Error > { t . parse () } }
};
}
