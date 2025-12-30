// Generated macro for impl_553 (impl)
macro_rules! Depcrate_uriimpl_553 {
() => {
// Module: crate::uri
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a String > for Uri { type Error = InvalidUri ; # [inline] fn try_from (t : & 'a String) -> Result < Self , Self :: Error > { t . parse () } }
};
}
