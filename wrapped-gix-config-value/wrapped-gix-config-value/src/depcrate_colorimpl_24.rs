// Generated macro for impl_24 (impl)
macro_rules! Depcrate_colorimpl_24 {
() => {
// Module: crate::color
// Provides: {"impl_24"}
// Dependencies: {}
impl TryFrom < Cow < '_ , BStr > > for Color { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
};
}
