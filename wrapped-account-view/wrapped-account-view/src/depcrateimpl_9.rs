// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : ? Sized > Deref for Ref < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { self . value . as_ref () } } }
};
}
