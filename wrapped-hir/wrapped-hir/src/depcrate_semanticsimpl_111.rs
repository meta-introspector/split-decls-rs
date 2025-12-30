// Generated macro for impl_111 (impl)
macro_rules! Depcrate_semanticsimpl_111 {
() => {
// Module: crate::semantics
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'db , DB : ? Sized > ops :: Deref for Semantics < 'db , DB > { type Target = SemanticsImpl < 'db > ; fn deref (& self) -> & Self :: Target { & self . imp } }
};
}
