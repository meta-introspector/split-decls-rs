// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl core :: ops :: Drop for Rc4State { fn drop (& mut self) { self . state . zeroize () ; self . i . zeroize () ; self . j . zeroize () ; } }
};
}
