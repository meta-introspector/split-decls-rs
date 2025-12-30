// Generated macro for impl_79 (impl)
macro_rules! Depcrate_saltimpl_79 {
() => {
// Module: crate::salt
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for Salt < 'a > { type Error = Error ; fn try_from (input : & 'a str) -> Result < Self > { Self :: from_b64 (input) } }
};
}
