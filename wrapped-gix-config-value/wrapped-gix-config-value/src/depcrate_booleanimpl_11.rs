// Generated macro for impl_11 (impl)
macro_rules! Depcrate_booleanimpl_11 {
() => {
// Module: crate::boolean
// Provides: {"impl_11"}
// Dependencies: {}
impl TryFrom < Cow < '_ , BStr > > for Boolean { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
};
}
