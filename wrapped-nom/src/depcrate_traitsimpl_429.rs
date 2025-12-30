// Generated macro for impl_429 (impl)
macro_rules! Depcrate_traitsimpl_429 {
() => {
// Module: crate::traits
// Provides: {"impl_429"}
// Dependencies: {}
impl < 'a , R : FromStr > ParseTo < R > for & 'a [u8] { fn parse_to (& self) -> Option < R > { from_utf8 (self) . ok () . and_then (| s | s . parse () . ok ()) } }
};
}
