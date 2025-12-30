// Generated macro for impl_116 (impl)
macro_rules! Depcrateimpl_116 {
() => {
// Module: crate
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < R : Rounds , V : Variant > Drop for ChaChaCore < R , V > { fn drop (& mut self) { self . state . zeroize () ; } }
};
}
