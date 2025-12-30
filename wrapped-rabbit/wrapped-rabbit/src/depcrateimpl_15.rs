// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl core :: ops :: Drop for State { fn drop (& mut self) { self . x . zeroize () ; self . c . zeroize () ; self . carry_bit . zeroize () ; } }
};
}
