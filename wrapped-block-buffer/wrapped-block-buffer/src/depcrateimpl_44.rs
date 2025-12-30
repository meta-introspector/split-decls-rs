// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < BS : ArraySize , K : BufferKind > Drop for BlockBuffer < BS , K > { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] self . zeroize () ; } }
};
}
