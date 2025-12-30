// Generated macro for impl_33 (impl)
macro_rules! Depcrate_stateimpl_33 {
() => {
// Module: crate::state
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " Initialization"] impl < 'a > StateRef < 'a > { # [doc = " Keep `input` in one of our enums."] pub fn from_bytes (input : & 'a [u8]) -> Self { Self :: Value (ValueRef :: from_bytes (input)) } }
};
}
