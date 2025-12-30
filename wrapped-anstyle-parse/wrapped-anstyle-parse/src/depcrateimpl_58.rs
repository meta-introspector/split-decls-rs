// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "utf8")] impl utf8 :: Receiver for VtUtf8Receiver < '_ > { fn codepoint (& mut self , c : char) { * self . 0 = Some (c) ; } fn invalid_sequence (& mut self) { * self . 0 = Some ('�') ; } }
};
}
