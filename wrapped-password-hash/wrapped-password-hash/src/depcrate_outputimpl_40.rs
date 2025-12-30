// Generated macro for impl_40 (impl)
macro_rules! Depcrate_outputimpl_40 {
() => {
// Module: crate::output
// Provides: {"impl_40"}
// Dependencies: {}
impl FromStr for Output { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: b64_decode (s) } }
};
}
