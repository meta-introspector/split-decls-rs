// Generated macro for impl_79 (impl)
macro_rules! Depcrate_counterimpl_79 {
() => {
// Module: crate::counter
// Provides: {"impl_79"}
// Dependencies: {}
impl < C > ops :: Deref for Sender < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }
};
}
