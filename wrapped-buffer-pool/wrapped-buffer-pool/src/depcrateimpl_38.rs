// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : Default + Reuse > Deref for Pooled < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
