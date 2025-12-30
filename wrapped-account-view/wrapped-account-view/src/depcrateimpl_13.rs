// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : ? Sized > Deref for RefMut < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { self . value . as_ref () } } }
};
}
