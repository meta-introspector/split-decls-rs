// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl TryFrom < & str > for DerivationPath { type Error = DerivationPathError ; fn try_from (s : & str) -> Result < Self , Self :: Error > { Self :: from_key_str (s) } }
};
}
