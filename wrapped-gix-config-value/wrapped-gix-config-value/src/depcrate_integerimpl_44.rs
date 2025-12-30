// Generated macro for impl_44 (impl)
macro_rules! Depcrate_integerimpl_44 {
() => {
// Module: crate::integer
// Provides: {"impl_44"}
// Dependencies: {}
impl TryFrom < Cow < '_ , BStr > > for Integer { type Error = Error ; fn try_from (c : Cow < '_ , BStr >) -> Result < Self , Self :: Error > { Self :: try_from (c . as_ref ()) } }
};
}
