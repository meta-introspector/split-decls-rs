// Generated macro for impl_515 (impl)
macro_rules! Depcrate_ixdtfimpl_515 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_515"}
// Dependencies: {}
impl FromStr for Date < Iso > { type Err = ParseError ; fn from_str (rfc_9557_str : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (rfc_9557_str , Iso) } }
};
}
