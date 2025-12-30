// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < BS : ArraySize , K : BufferKind > Zeroize for BlockBuffer < BS , K > { # [inline] fn zeroize (& mut self) { self . buffer . zeroize () ; self . pos . zeroize () ; } }
};
}
