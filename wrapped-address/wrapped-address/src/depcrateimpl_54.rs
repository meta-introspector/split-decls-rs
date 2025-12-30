// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "decode")] impl TryFrom < & str > for Address { type Error = ParseAddressError ; fn try_from (s : & str) -> Result < Self , Self :: Error > { Address :: from_str (s) } }
};
}
